use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bit_vec::BitVec;
use rsiot::{
    components_config::{
        master_device::{self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master::{FieldbusRequest, FieldbusResponse},
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
    pub update_period: Duration,
    pub fn_output: fn(&mut Buffer) -> Vec<Message<TMsg>>,
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
            fn_init_requests: |_| {
                let init_req = FieldbusRequest::new(
                    RequestKind::Init,
                    vec![MCP23S17::write_iodir_a(0xFF), MCP23S17::write_iodir_b(0xFF)],
                );

                vec![init_req]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.update_period,
                fn_requests: |_| {
                    let req_read_inputs = FieldbusRequest::new(
                        RequestKind::ReadInputs,
                        vec![MCP23S17::read_gpio_a(), MCP23S17::read_gpio_b()],
                    );

                    Ok(vec![req_read_inputs])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.into();

                if let RequestKind::ReadInputs = request_kind {
                    let di_b = &response.payload[0];
                    let di_b = BitVec::from_bytes(di_b);
                    buffer.read.input_states.dib_0 = di_b[0];
                    buffer.read.input_states.dib_1 = di_b[1];
                    buffer.read.input_states.dib_2 = di_b[2];
                    buffer.read.input_states.dib_3 = di_b[3];
                    buffer.read.input_states.dib_4 = di_b[4];
                    buffer.read.input_states.dib_5 = di_b[5];
                    buffer.read.input_states.dib_6 = di_b[6];
                    buffer.read.input_states.dib_7 = di_b[7];

                    let di_a = &response.payload[1];
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
    pub read: Read,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub input_states: InputStates,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputStates {
    pub dia_0: bool,
    pub dia_1: bool,
    pub dia_2: bool,
    pub dia_3: bool,
    pub dia_4: bool,
    pub dia_5: bool,
    pub dia_6: bool,
    pub dia_7: bool,
    pub dib_0: bool,
    pub dib_1: bool,
    pub dib_2: bool,
    pub dib_3: bool,
    pub dib_4: bool,
    pub dib_5: bool,
    pub dib_6: bool,
    pub dib_7: bool,
}

impl BufferBound for Buffer {}

#[derive(FromRepr)]
pub enum RequestKind {
    Init,
    ReadInputs,
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
