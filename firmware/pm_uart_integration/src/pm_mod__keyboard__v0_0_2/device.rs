use std::time::Duration;

use async_trait::async_trait;
use rsiot::{
    components_config::{
        master_device::{ConfigPeriodicRequest, DeviceBase, DeviceTrait, Result},
        uart_general::{
            protocol::{Protocol, UartPacket},
            FieldbusRequest, FieldbusResponse,
        },
    },
    message::{Message, MsgDataBound},
};
use tokio::sync::{broadcast, mpsc};
use tracing::warn;

use super::{Buffer, Request, Response};

/// Тестовое устройство
#[derive(Clone, Debug)]
pub struct Device<TMsg> {
    /// Адрес
    pub address: u8,

    pub periodic_request: Duration,

    /// Преобразование данных из буфера в исходящие сообщения
    pub fn_output: fn(&mut Buffer) -> Vec<Message<TMsg>>,
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse> for Device<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> Result<()> {
        let device = DeviceBase {
            fn_init_requests: |_| vec![],
            buffer_default: Buffer {
                protocol: Protocol::new(self.address),
                ..Default::default()
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.periodic_request,
                fn_requests: |buffer| {
                    let req = Request::GetData;
                    let req = buffer.protocol.serialize_request(req)?;
                    Ok(vec![req])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            fn_buffer_to_request: |_buffer| Ok(vec![]),
            fn_response_to_buffer,
            fn_buffer_to_msgs: self.fn_output,
        };
        device
            .spawn(
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await?;
        Ok(())
    }
}

fn fn_response_to_buffer(response: FieldbusResponse, buffer: &mut Buffer) -> anyhow::Result<()> {
    let response: UartPacket<Response> = buffer.protocol.deserialize_response(response)?;

    if response.address != buffer.protocol.address {
        warn!("Wrong address");
        return Ok(());
    }

    match response.data {
        Response::Data {
            pressed_button,
            pressed_touch,
        } => {
            buffer.read.pressed_button = pressed_button;
            buffer.read.pressed_touch = pressed_touch;
        }
    }

    Ok(())
}
