use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bitvec::{order::Msb0, view::BitView};
use rsiot::{
    components_config::{
        master_device::{self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master,
    },
    executor::MsgBusInput,
    message::{Message, MsgDataBound},
};
use tokio::sync::mpsc;

use crate::chips::MCP23S17;

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(&TMsg, &mut Buffer),
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
        ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
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
                    vec![MCP23S17::write_iodir_a(0x00), MCP23S17::write_iodir_b(0x00)],
                )]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(1000),
                fn_requests: |buffer| {
                    let reg_a = buffer.write.reg_a();
                    let reg_b = buffer.write.reg_b();
                    Ok(vec![spi_master::FieldbusRequest::new(
                        RequestKind::SetOutputs,
                        vec![MCP23S17::write_gpio_a(reg_a), MCP23S17::write_gpio_b(reg_b)],
                    )])
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |buffer| {
                let reg_a = buffer.write.reg_a();
                let reg_b = buffer.write.reg_b();
                Ok(vec![spi_master::FieldbusRequest::new(
                    RequestKind::SetOutputs,
                    vec![MCP23S17::write_gpio_a(reg_a), MCP23S17::write_gpio_b(reg_b)],
                )])
            },
            fn_response_to_buffer: |_, _| Ok(false),
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Buffer::default(),
        };
        device
            .spawn(
                "dq16src".to_string(),
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await?;
        Err(master_device::Error::EndExecution)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub write: Write,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Write {
    pub dqa_0: bool,
    pub dqa_1: bool,
    pub dqa_2: bool,
    pub dqa_3: bool,
    pub dqa_4: bool,
    pub dqa_5: bool,
    pub dqa_6: bool,
    pub dqa_7: bool,
    pub dqb_0: bool,
    pub dqb_1: bool,
    pub dqb_2: bool,
    pub dqb_3: bool,
    pub dqb_4: bool,
    pub dqb_5: bool,
    pub dqb_6: bool,
    pub dqb_7: bool,
}
impl Write {
    pub fn reg_a(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Msb0>();
        byte_view.set(0, self.dqb_0);
        byte_view.set(1, self.dqb_1);
        byte_view.set(2, self.dqb_2);
        byte_view.set(3, self.dqb_3);
        byte_view.set(4, self.dqb_4);
        byte_view.set(5, self.dqb_5);
        byte_view.set(6, self.dqb_6);
        byte_view.set(7, self.dqb_7);
        byte
    }

    pub fn reg_b(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Msb0>();
        byte_view.set(0, self.dqa_0);
        byte_view.set(1, self.dqa_1);
        byte_view.set(2, self.dqa_2);
        byte_view.set(3, self.dqa_3);
        byte_view.set(4, self.dqa_4);
        byte_view.set(5, self.dqa_5);
        byte_view.set(6, self.dqa_6);
        byte_view.set(7, self.dqa_7);
        byte
    }
}

impl BufferBound for Buffer {}

#[repr(u8)]
pub enum RequestKind {
    Init,
    SetOutputs,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}
