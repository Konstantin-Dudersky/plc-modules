use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bit_vec::BitVec;
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

use crate::{chips::mcp23017::MCP23017, device_id_i2c};

use super::{Buffer, DEVICE_NAME, request_kind::RequestKind};

/// Модуль PMCNV_DIx16
#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес чипа MCP23017
    ///
    /// | A2 | A1 | A0 | Адрес |
    /// |---|---|---|------|
    /// | - | - | - | 0x20 |
    /// | - | - | + | 0x21 |
    /// | - | + | - | 0x22 |
    /// | - | + | + | 0x23 |
    /// | + | - | - | 0x24 |
    /// | + | - | + | 0x25 |
    /// | + | + | - | 0x26 |
    /// | + | + | + | 0x27 |
    pub address: I2cAddress,

    /// Период запроса данных
    pub update_period: Duration,

    /// Функция создания сообщений с данными модуля
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
            fn_init_requests: |buffer| {
                let req_init = FieldbusRequest::new(
                    buffer.address,
                    RequestKind::Init,
                    vec![MCP23017::write_iodir_a(0xFF), MCP23017::write_iodir_b(0xFF)],
                );
                vec![req_init]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.update_period,
                fn_requests: |buffer| {
                    let req_read = FieldbusRequest::new(
                        buffer.address,
                        RequestKind::ReadInputs,
                        vec![MCP23017::read_gpio_a(), MCP23017::read_gpio_b()],
                    );
                    Ok(vec![req_read])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.try_into()?;

                let payload = match response.payload {
                    Ok(payload) => payload,
                    Err(err) => {
                        return ResponseResult::error(err);
                    }
                };

                match request_kind {
                    RequestKind::Init => ResponseResult::ok_init_completed(),

                    RequestKind::ReadInputs => {
                        let di_b = &payload[0];
                        let di_b = BitVec::from_bytes(di_b);

                        buffer.read.input_states.ch08 = di_b[0];
                        buffer.read.input_states.ch09 = di_b[1];
                        buffer.read.input_states.ch10 = di_b[2];
                        buffer.read.input_states.ch11 = di_b[3];
                        buffer.read.input_states.ch12 = di_b[4];
                        buffer.read.input_states.ch13 = di_b[5];
                        buffer.read.input_states.ch14 = di_b[6];
                        buffer.read.input_states.ch15 = di_b[7];

                        let di_a = &payload[1];
                        let di_a = BitVec::from_bytes(di_a);
                        buffer.read.input_states.ch00 = di_a[0];
                        buffer.read.input_states.ch01 = di_a[1];
                        buffer.read.input_states.ch02 = di_a[2];
                        buffer.read.input_states.ch03 = di_a[3];
                        buffer.read.input_states.ch04 = di_a[4];
                        buffer.read.input_states.ch05 = di_a[5];
                        buffer.read.input_states.ch06 = di_a[6];
                        buffer.read.input_states.ch07 = di_a[7];
                        ResponseResult::ok()
                    }
                }
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                address: self.address,
                ..Default::default()
            },
        };
        device
            .spawn(
                device_id_i2c(DEVICE_NAME, self.address),
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
