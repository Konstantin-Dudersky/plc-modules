use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bit_vec::BitVec;
use rsiot::{
    components_config::{
        i2c_master::{FieldbusRequest, FieldbusResponse},
        master_device::{self, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
    },
    executor::MsgBusInput,
    message::{Message, MsgDataBound},
};
use tokio::sync::mpsc;
use tracing::warn;

use crate::chips::mcp23s17_i2c::MCP23S17;

use super::{request_kind::RequestKind, Buffer};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub update_period: Duration,
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
    ) -> master_device::Result<()> {
        let device: DeviceBase<TMsg, FieldbusRequest, FieldbusResponse, Buffer> = DeviceBase {
            fn_init_requests: |buffer| {
                let req_init = FieldbusRequest::new(
                    buffer.address,
                    RequestKind::Init,
                    vec![MCP23S17::write_iodir_a(0xFF), MCP23S17::write_iodir_b(0xFF)],
                );
                vec![req_init]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.update_period,
                fn_requests: |buffer| {
                    let req_read = FieldbusRequest::new(
                        buffer.address,
                        RequestKind::ReadInputs,
                        vec![MCP23S17::read_gpio_a(), MCP23S17::read_gpio_b()],
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
                        warn!("Error reading DI16: {}", err);
                        return Ok(false);
                    }
                };

                if let RequestKind::ReadInputs = request_kind {
                    let di_b = &payload[0];
                    let di_b = BitVec::from_bytes(di_b);

                    buffer.read.input_states.dib_0 = di_b[0];
                    buffer.read.input_states.dib_1 = di_b[1];
                    buffer.read.input_states.dib_2 = di_b[2];
                    buffer.read.input_states.dib_3 = di_b[3];
                    buffer.read.input_states.dib_4 = di_b[4];
                    buffer.read.input_states.dib_5 = di_b[5];
                    buffer.read.input_states.dib_6 = di_b[6];
                    buffer.read.input_states.dib_7 = di_b[7];

                    let di_a = &payload[1];
                    let di_a = BitVec::from_bytes(di_a);
                    buffer.read.input_states.dia_0 = di_a[0];
                    buffer.read.input_states.dia_1 = di_a[1];
                    buffer.read.input_states.dia_2 = di_a[2];
                    buffer.read.input_states.dia_3 = di_a[3];
                    buffer.read.input_states.dia_4 = di_a[4];
                    buffer.read.input_states.dia_5 = di_a[5];
                    buffer.read.input_states.dia_6 = di_a[6];
                    buffer.read.input_states.dia_7 = di_a[7];
                }

                Ok(false)
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                address: self.address,
                ..Default::default()
            },
        };
        device
            .spawn(
                "di16".to_string(),
                ch_rx_msgbus_to_device,
                ch_tx_device_to_fieldbus,
                ch_rx_fieldbus_to_device,
                ch_tx_device_to_msgbus,
            )
            .await?;
        Err(master_device::Error::EndExecution)
    }
}
