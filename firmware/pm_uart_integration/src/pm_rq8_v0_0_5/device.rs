use rsiot::components_config::uart_general::protocol::Protocol;
use tracing::warn;

use super::{
    async_trait, broadcast, mpsc, Buffer, ConfigPeriodicRequest, DeviceBase, DeviceTrait, Duration,
    FieldbusRequest, FieldbusResponse, Message, MsgDataBound, Request,
};

/// Тестовое устройство
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct Device<TMsg> {
    /// Адрес
    pub address: u8,

    pub periodic_request: Duration,

    /// Преобразование входящих сообщений в данные для сохранения в буфере
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),

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
    ) -> super::Result<()> {
        let device = DeviceBase {
            fn_init_requests: |_| vec![],
            buffer_default: Buffer {
                protocol: Protocol::new(self.address),
                ..Default::default()
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.periodic_request,
                fn_requests: |buffer| Ok(buffer_to_requests(buffer)),
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |buffer| Ok(buffer_to_requests(buffer)),
            fn_response_to_buffer: |_response: FieldbusResponse, _buffer: &mut Buffer| Ok(()),
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

fn buffer_to_requests(buffer: &Buffer) -> Vec<FieldbusRequest> {
    [
        Request::SetOutputs(buffer.outputs_to_u8()),
        Request::SetMasterLiveCounter(buffer.master_live_counter),
    ]
    .iter()
    .filter_map(|r| {
        let r = buffer.protocol.serialize_request(r);
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("Failed to create request: {:?}", e);
                None
            }
        }
    })
    .collect()
}
