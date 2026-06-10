use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bitvec::{order::Lsb0, view::BitView};
use rsiot::{
    components_config::{
        i2c_master::{FieldbusRequest, FieldbusResponse, I2cAddress},
        master_device::{
            self, ConfigPeriodicRequest, DeviceBase, DeviceTrait, FieldbusDiagMsg, ResponseResult,
        },
    },
    executor::MsgBusInput,
    message::{Message, MsgDataBound},
};
use tokio::sync::mpsc;
use tracing::warn;

use crate::chips::mcp23017::MCP23017;

use super::{
    Buffer, DEVICE_NAME,
    buffer::{PressedButton, Write},
    request_kind::RequestKind,
};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: I2cAddress,
    pub fn_input: fn(&TMsg, &mut Buffer),
    pub fn_output: fn(&mut Buffer) -> Vec<TMsg>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse> for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
        ch_tx_device_to_diag: mpsc::Sender<FieldbusDiagMsg>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<TMsg, FieldbusRequest, FieldbusResponse, Buffer> = DeviceBase {
            fn_init_requests,
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(100),
                fn_requests: |buffer| {
                    let reg_a = buffer.write.reg_a();
                    let reg_b = buffer.write.reg_b();

                    let req = FieldbusRequest::new(
                        buffer.address,
                        RequestKind::ReadButton,
                        vec![
                            MCP23017::write_gpio_a(reg_a),
                            MCP23017::write_gpio_b(reg_b),
                            MCP23017::read_gpio_a(),
                            MCP23017::read_gpio_b(),
                        ],
                    );

                    Ok(vec![req])
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request,
            fn_response_to_buffer: |response, buffer| {
                let kind: RequestKind = response.request_kind.try_into()?;

                let payload = match response.payload {
                    Ok(payload) => payload,
                    Err(err) => {
                        warn!("Error reading state: {}", err);
                        return ResponseResult::error(err);
                    }
                };

                match kind {
                    RequestKind::Init => ResponseResult::ok_init_completed(),

                    RequestKind::SetLed => ResponseResult::ok(),

                    RequestKind::ReadButton => {
                        let a = payload[2].view_bits::<Lsb0>();
                        let b = payload[3].view_bits::<Lsb0>();

                        let row_0 = a.get(0).unwrap();
                        let col_0 = a.get(1).unwrap();
                        let col_1 = a.get(2).unwrap();
                        let col_2 = a.get(3).unwrap();
                        let col_3 = b.get(4).unwrap();
                        let col_4 = b.get(5).unwrap();
                        let col_5 = b.get(6).unwrap();
                        let row_1 = b.get(7).unwrap();

                        let row = if !row_0 {
                            0
                        } else if !row_1 {
                            1
                        } else {
                            -1
                        };

                        let col = if !col_0 {
                            0
                        } else if !col_1 {
                            1
                        } else if !col_2 {
                            2
                        } else if !col_3 {
                            3
                        } else if !col_4 {
                            4
                        } else if !col_5 {
                            5
                        } else {
                            -1
                        };

                        buffer.read.pressed_button = match (row, col) {
                            (0, 0) => PressedButton::Row0Col0,
                            (0, 1) => PressedButton::Row0Col1,
                            (0, 2) => PressedButton::Row0Col2,
                            (0, 3) => PressedButton::Row0Col3,
                            (0, 4) => PressedButton::Row0Col4,
                            (0, 5) => PressedButton::Row0Col5,
                            (1, 0) => PressedButton::Row1Col0,
                            (1, 1) => PressedButton::Row1Col1,
                            (1, 2) => PressedButton::Row1Col2,
                            (1, 3) => PressedButton::Row1Col3,
                            (1, 4) => PressedButton::Row1Col4,
                            (1, 5) => PressedButton::Row1Col5,
                            _ => PressedButton::None,
                        };

                        if matches!(buffer.read.pressed_button, PressedButton::None) {
                            // Переключаем строку для чтения
                            buffer.read_row += 1;
                            if buffer.read_row >= 2 {
                                buffer.read_row = 0;
                            }
                        }

                        buffer.write.button_row0 = buffer.read_row != 0;
                        buffer.write.button_row1 = buffer.read_row != 1;
                        ResponseResult::ok()
                    }
                }
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                address: self.address,
                write: Write {
                    button_row0: true,
                    button_row1: false,
                    ..Default::default()
                },
                ..Default::default()
            },
        };
        device
            .spawn(
                format!("{} ({:x?})", DEVICE_NAME, self.address),
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
                ch_tx_device_to_diag,
            )
            .await?;
        Err(master_device::Error::EndExecution)
    }
}

pub fn fn_init_requests(buffer: &Buffer) -> Vec<FieldbusRequest> {
    vec![FieldbusRequest::new(
        buffer.address,
        RequestKind::Init,
        vec![
            MCP23017::write_iodir_a(0b00001110),
            MCP23017::write_iodir_b(0b01110000),
            MCP23017::write_gppua(0b00001110),
            MCP23017::write_gppub(0b01110000),
        ],
    )]
}

pub fn fn_buffer_to_request(buffer: &Buffer) -> anyhow::Result<Vec<FieldbusRequest>> {
    let reg_a = buffer.write.reg_a();
    let reg_b = buffer.write.reg_b();

    Ok(vec![FieldbusRequest::new(
        buffer.address,
        RequestKind::SetLed,
        vec![MCP23017::write_gpio_a(reg_a), MCP23017::write_gpio_b(reg_b)],
    )])
}
