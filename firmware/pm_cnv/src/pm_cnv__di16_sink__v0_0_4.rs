use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use bit_vec::BitVec;
use rsiot::{
    components_config::{
        master_device::{self, BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait},
        spi_master,
    },
    message::{Message, MsgDataBound},
};
use strum::FromRepr;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

use crate::chips::MCP23S17;

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub fn_output: fn(&mut Buffer) -> Vec<Message<TMsg>>,
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
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
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
                    vec![MCP23S17::write_iodir_a(0xFF), MCP23S17::write_iodir_b(0xFF)],
                )]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(1000),
                fn_requests: |_| {
                    Ok(vec![spi_master::FieldbusRequest::new(
                        RequestKind::ReadInputs,
                        vec![MCP23S17::read_gpio_a(), MCP23S17::read_gpio_b()],
                    )])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let request_kind: RequestKind = response.request_kind.into();
                let el = response.request_creation_time.elapsed();
                info!("Response time: {:?}", el);

                if let RequestKind::ReadInputs = request_kind {
                    let mut all = [false; 16];

                    let di_a = &response.payload[0];
                    let di_a = BitVec::from_bytes(di_a);
                    all[0] = di_a[0];
                    all[1] = di_a[1];
                    all[2] = di_a[2];
                    all[3] = di_a[3];
                    all[4] = di_a[4];
                    all[5] = di_a[5];
                    all[6] = di_a[6];
                    all[7] = di_a[7];

                    let di_b = &response.payload[1];
                    let di_b = BitVec::from_bytes(di_b);
                    all[8] = di_b[0];
                    all[9] = di_b[1];
                    all[10] = di_b[2];
                    all[11] = di_b[3];
                    all[12] = di_b[4];
                    all[13] = di_b[5];
                    all[14] = di_b[6];
                    all[15] = di_b[7];

                    buffer.all_inputs = all;
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
    pub all_inputs: [bool; 16],
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
