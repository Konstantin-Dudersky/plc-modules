use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use rsiot::{
    components_config::{
        i2c_master::{FieldbusRequest, FieldbusResponse},
        master_device::{self, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
    },
    message::{Message, MsgDataBound},
};
use tokio::sync::{broadcast, mpsc};

use crate::chips::mcp23s17_i2c::MCP23S17;

use super::{request_kind::RequestKind, Buffer};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub fn_input: fn(&TMsg, &mut Buffer),
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse> for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<TMsg, FieldbusRequest, FieldbusResponse, Buffer> = DeviceBase {
            fn_init_requests,
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(1000),
                fn_requests: |buffer| {
                    let reg_a = buffer.write.reg_a();
                    let reg_b = buffer.write.reg_b();
                    Ok(vec![FieldbusRequest::new(
                        buffer.address,
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
                Ok(vec![FieldbusRequest::new(
                    buffer.address,
                    RequestKind::SetOutputs,
                    vec![MCP23S17::write_gpio_a(reg_a), MCP23S17::write_gpio_b(reg_b)],
                )])
            },
            fn_response_to_buffer: |_, _| Ok(false),
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Buffer {
                address: self.address,
                ..Default::default()
            },
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

pub fn fn_init_requests(buffer: &Buffer) -> Vec<FieldbusRequest> {
    vec![FieldbusRequest::new(
        buffer.address,
        RequestKind::Init,
        vec![MCP23S17::write_iodir_a(0x00), MCP23S17::write_iodir_b(0x00)],
    )]
}
