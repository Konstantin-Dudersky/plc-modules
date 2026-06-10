use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use rsiot::{
    components_config::{
        i2c_master::{FieldbusRequest, FieldbusResponse, I2cAddress, Operation},
        master_device::{self, DeviceBase, DeviceTrait, FieldbusDiagMsg, ResponseResult},
    },
    executor::MsgBusInput,
    message::{Message, MsgDataBound},
};
use tokio::sync::mpsc;
use tracing::warn;

use crate::chips::pca9685::PCA9685;

use super::{Buffer, DEVICE_NAME, buffer::Write, request_kind::RequestKind};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес устройства на шине I2C
    ///
    /// Определяется состоянием пинов A0 - A5. Определяется | 1 | A5 | A4 | A3 | A2 | A1 | A0 |
    ///
    /// Если все подтянуты к GND, то адрес равен 0x40.
    ///
    /// Также для всех чипов по-умолчанию есть неизменяемый адрес 0x70.
    pub address: I2cAddress,

    /// Частота обновления, [Гц]
    ///
    /// По умолчанию 200 Гц. Допустимые значения от 24 Гц до 1526 Гц.
    pub update_rate: f32,

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
        ch_rx_msgbus_to_device: MsgBusInput<TMsg>,
        ch_tx_device_to_fieldbus: mpsc::Sender<FieldbusRequest>,
        ch_rx_fieldbus_to_device: mpsc::Receiver<FieldbusResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
        ch_tx_device_to_diag: mpsc::Sender<FieldbusDiagMsg>,
    ) -> master_device::Result<()> {
        let device: DeviceBase<TMsg, FieldbusRequest, FieldbusResponse, Buffer> = DeviceBase {
            fn_init_requests,
            periodic_requests: vec![],
            fn_msgs_to_buffer: self.fn_input,
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request,
            fn_response_to_buffer: |response, _buffer| {
                let kind: RequestKind = response.request_kind.try_into()?;

                let _ = match response.payload {
                    Ok(payload) => payload,
                    Err(err) => {
                        warn!("Error reading state: {}", err);
                        return ResponseResult::error(err);
                    }
                };

                match kind {
                    RequestKind::Init => ResponseResult::ok_init_completed(),
                    _ => ResponseResult::ok(),
                }
            },
            fn_buffer_to_msgs: |_| vec![],
            buffer_default: Buffer {
                address: self.address,
                update_rate: self.update_rate,
                write: Write {
                    ..Default::default()
                },
            },
        };
        device
            .spawn(
                format!("{} ({:x?})", DEVICE_NAME, self.address),
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

pub fn fn_init_requests(buffer: &Buffer) -> Vec<FieldbusRequest> {
    let write_led0_15 = PCA9685::write_led0_15(vec![0x00; 64]).unwrap();

    vec![
        FieldbusRequest::new(
            I2cAddress::Direct { address: 0x00 },
            RequestKind::SoftwareReset,
            vec![Operation::Write {
                write_data: vec![0x06],
            }],
        ),
        FieldbusRequest::new(
            buffer.address,
            RequestKind::Init,
            vec![
                PCA9685::write_mode1(0x10),
                PCA9685::write_prescale(buffer.prescale()),
                PCA9685::write_mode1(0x20),
                PCA9685::write_mode2(0b0000_0100),
                write_led0_15,
            ],
        ),
    ]
}

pub fn fn_buffer_to_request(buffer: &Buffer) -> Result<Vec<FieldbusRequest>, anyhow::Error> {
    let mut data = Vec::with_capacity(64);

    data.extend_from_slice(&buffer.write.ch15.registers());
    data.extend_from_slice(&buffer.write.ch14.registers());
    data.extend_from_slice(&buffer.write.ch13.registers());
    data.extend_from_slice(&buffer.write.ch12.registers());
    data.extend_from_slice(&buffer.write.ch11.registers());
    data.extend_from_slice(&buffer.write.ch10.registers());
    data.extend_from_slice(&buffer.write.ch09.registers());
    data.extend_from_slice(&buffer.write.ch08.registers());
    data.extend_from_slice(&buffer.write.ch07.registers());
    data.extend_from_slice(&buffer.write.ch06.registers());
    data.extend_from_slice(&buffer.write.ch05.registers());
    data.extend_from_slice(&buffer.write.ch04.registers());
    data.extend_from_slice(&buffer.write.ch03.registers());
    data.extend_from_slice(&buffer.write.ch02.registers());
    data.extend_from_slice(&buffer.write.ch01.registers());
    data.extend_from_slice(&buffer.write.ch00.registers());

    Ok(vec![FieldbusRequest::new(
        buffer.address,
        RequestKind::SetChannels,
        vec![PCA9685::write_led0_15(data)?],
    )])
}
