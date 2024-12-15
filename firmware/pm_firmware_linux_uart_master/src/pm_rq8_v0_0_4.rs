//! Пример обмена данными с тестовым устройством. Реализацию см. `rsiot-examples`.

use std::time::Duration;

use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc};

use rsiot::components_config::uart_general::{BufferBound, UartMessageRaw};
use rsiot::components_config::uart_master::{ConfigPeriodicRequest, DeviceBase, DeviceTrait};
use rsiot::message::{Message, MsgDataBound};

use pm_firmware_uart_shared::pm_rq8_v0_0_4::{UartRequest, UartResponse};

/// Тестовое устройство
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct PM_RQ8_v0_0_4<TMsg> {
    /// Адрес
    pub address: u8,

    /// Преобразование входящих сообщений в данные для сохранения в буфере
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),

    /// Преобразование данных из буфера в исходящие сообщения
    pub fn_output: fn(&Buffer) -> Vec<Message<TMsg>>,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, const MESSAGE_LEN: usize> DeviceTrait<TMsg, MESSAGE_LEN> for PM_RQ8_v0_0_4<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw<MESSAGE_LEN>>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw<MESSAGE_LEN>>,
        ch_tx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_rx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()> {
        let device = DeviceBase {
            address: self.address,
            buffer_default: Buffer::default(),
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(500),
                fn_request: |buffer| UartRequest::SetOutputs((*buffer).into()),
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |_buffer: &Buffer| vec![],
            fn_response_to_buffer: |_response: UartResponse, _buffer: &mut Buffer| {},
            fn_buffer_to_msgs: self.fn_output,
        };
        device
            .spawn(
                ch_tx_device_to_uart,
                ch_rx_uart_to_device,
                ch_tx_msgbus_to_device,
                ch_rx_device_to_msgbus,
            )
            .await?;
        Ok(())
    }
}

/// Буфер данных коммуникации с модулем PM-RQ8
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Buffer {
    /// Состояние выхода 0
    pub output_0: bool,
    /// Состояние выхода 1
    pub output_1: bool,
    /// Состояние выхода 2
    pub output_2: bool,
    /// Состояние выхода 3
    pub output_3: bool,
    /// Состояние выхода 4
    pub output_4: bool,
    /// Состояние выхода 5
    pub output_5: bool,
    /// Состояние выхода 6
    pub output_6: bool,
    /// Состояние выхода 7
    pub output_7: bool,
}

impl BufferBound for Buffer {}

impl From<Buffer> for u8 {
    fn from(value: Buffer) -> Self {
        let mut sum = 0;
        if value.output_0 {
            sum += 2_u8.pow(0);
        }
        if value.output_1 {
            sum += 2_u8.pow(1);
        }
        if value.output_2 {
            sum += 2_u8.pow(2);
        }
        if value.output_3 {
            sum += 2_u8.pow(3);
        }
        if value.output_4 {
            sum += 2_u8.pow(4);
        }
        if value.output_5 {
            sum += 2_u8.pow(5);
        }
        if value.output_6 {
            sum += 2_u8.pow(6);
        }
        if value.output_7 {
            sum += 2_u8.pow(7);
        }
        sum
    }
}
