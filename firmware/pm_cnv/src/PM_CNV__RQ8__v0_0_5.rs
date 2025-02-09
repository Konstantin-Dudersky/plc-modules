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
use tokio::sync::{broadcast, mpsc};

use crate::chips::MCP23S17;

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: u8,
    pub fn_input: fn(&Message<TMsg>, &mut Buffer),
}

#[async_trait]
impl<TMsg> DeviceTrait<TMsg, spi_master::FieldbusRequest, spi_master::FieldbusResponse, u8>
    for Device<TMsg>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_fieldbus: mpsc::Sender<spi_master::FieldbusRequest>,
        ch_rx_fieldbus_to_device: broadcast::Receiver<spi_master::FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<
            TMsg,
            spi_master::FieldbusRequest,
            spi_master::FieldbusResponse,
            Buffer,
            u8,
        > = DeviceBase {
            address: self.address,
            fn_init_requests: || {
                vec![spi_master::FieldbusRequest::new(
                    RequestKind::Init,
                    vec![MCP23S17::write_iodir_a(0x00)],
                )]
            },
            periodic_requests: vec![ConfigPeriodicRequest {
                period: Duration::from_millis(1000),
                fn_requests: |buffer| {
                    let outputs = buffer.outputs_to_byte();
                    vec![spi_master::FieldbusRequest::new(
                        RequestKind::SetOutputs,
                        vec![MCP23S17::write_gpio_a(outputs)],
                    )]
                },
            }],
            fn_msgs_to_buffer: self.fn_input,
            fn_buffer_to_request: |_| vec![],
            fn_response_to_buffer: |_, _| (),
            fn_buffer_to_msgs: |_| vec![],
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
    pub outputs: [bool; 8],
}

impl Buffer {
    fn outputs_to_byte(&self) -> u8 {
        let mut bv = BitVec::from_elem(8, false);
        for i in 0..8 {
            bv.set(i, self.outputs[i]);
        }
        bv.to_bytes()[0]
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
