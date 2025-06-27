use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bit_vec::BitVec;
use rsiot::{
    components_config::{
        master_device::{self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master,
    },
    message::{Message, MsgDataBound},
};
use strum::FromRepr;
use tokio::sync::{broadcast, mpsc};

use crate::chips::MCP23S17;

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_output: fn(&mut Buffer) -> Vec<TMsg>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse>
    for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<spi_master::FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<spi_master::FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<
            TMsg,
            spi_master::FieldbusRequest,
            spi_master::FieldbusResponse,
            Buffer,
        > = DeviceBase {
            fn_init_requests: |_| {
                vec![spi_master::FieldbusRequest::new(
                    RequestKind::Init,
                    vec![
                        MCP23S17::write_iodir_a(0x00),
                        MCP23S17::write_iodir_b(0xFF),
                        MCP23S17::write_gppub(0xFF),
                    ],
                )]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(20),
                fn_requests: |_| {
                    Ok(vec![spi_master::FieldbusRequest::new(
                        RequestKind::CheckButtons,
                        vec![
                            MCP23S17::write_gpio_a(0b1111_1110),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1111_1101),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1111_1011),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1111_0111),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1110_1111),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1101_1111),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b1011_1111),
                            MCP23S17::read_gpio_b(),
                            MCP23S17::write_gpio_a(0b0111_1111),
                            MCP23S17::read_gpio_b(),
                        ],
                    )])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.into();

                match request_kind {
                    RequestKind::Init => (),
                    RequestKind::CheckButtons => {
                        let mut row: usize;

                        row = 0;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 1;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 2;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 3;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 4;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 5;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 6;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }

                        row = 7;
                        let col_index = check_buttons_in_row(response.payload[row][0]);
                        if let Some(col_index) = col_index {
                            buffer.pressed_button = Some((row as u8, col_index));
                            return Ok(());
                        }
                    }
                }

                Ok(())
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer::default(),
        };
        device
            .spawn(
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await
            .unwrap();
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub pressed_button: Option<(u8, u8)>,
}

impl BufferBound for Buffer {}

#[derive(FromRepr)]
pub enum RequestKind {
    Init,
    CheckButtons,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}

impl From<u8> for RequestKind {
    fn from(value: u8) -> Self {
        RequestKind::from_repr(value as usize).unwrap()
    }
}

fn check_buttons_in_row(row: u8) -> Option<u8> {
    let buttons_in_row = BitVec::from_bytes(&[row]);

    for i in 0..buttons_in_row.len() {
        if !buttons_in_row[i] {
            return Some(i as u8);
        }
    }
    None
}
