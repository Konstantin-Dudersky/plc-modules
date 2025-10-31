use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
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
    pub address: u8,
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
                    vec![MCP23S17::write_iodir_a(0x00)],
                )]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(500),
                fn_requests: |buffer| {
                    let outputs = buffer.outputs;
                    Ok(vec![spi_master::FieldbusRequest::new(
                        RequestKind::SetOutputs,
                        vec![MCP23S17::write_gpio_a(outputs)],
                    )])
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |buffer| {
                let outputs = buffer.outputs;
                Ok(vec![spi_master::FieldbusRequest::new(
                    RequestKind::SetOutputs,
                    vec![MCP23S17::write_gpio_a(outputs)],
                )])
            },
            fn_response_to_buffer: |_, _| Ok(false),
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Buffer::default(),
        };
        device
            .spawn(
                "rq8".to_string(),
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
    pub outputs: u8,
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
