use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
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
use tracing::{info, warn};

use crate::{
    PMCMV_INA226x4_v000001::buffer::calc_calibration_register,
    chips::ina226::{self, INA226},
};

use super::{
    AveragingMode, Buffer, ConversionTime, DEVICE_NAME, buffer, request_kind::RequestKind,
};

#[derive(Debug)]
pub struct Device<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес устройства на шине I2C
    ///
    /// Определяется состоянием пинов A1 и A0
    ///
    /// | A1  | A0  | Адрес |
    /// |-----|-----|-------|
    /// | GND | GND | 0x40 |
    /// | GND | VS  | 0x41 |
    /// | GND | SDA | 0x42 |
    /// | GND | SCL | 0x43 |
    /// | VS  | GND | 0x44 |
    /// | VS  | VS  | 0x45 |
    /// | VS  | SDA | 0x46 |
    /// | VS  | SCL | 0x47 |
    /// | SDA | GND | 0x48 |
    /// | SDA | VS  | 0x49 |
    /// | SDA | SDA | 0x4A |
    /// | SDA | SCL | 0x4B |
    /// | SCL | GND | 0x4C |
    /// | SCL | VS  | 0x4D |
    /// | SCL | SDA | 0x4E |
    /// | SCL | SCL | 0x4F |
    pub address: I2cAddress,

    /// Сопротивление шунта, [Ом]
    pub shunt_resistance: f64,

    /// Максимальный ожидаемый ток, [А]
    pub max_current: f64,

    pub average_mode: AveragingMode,

    pub bus_voltage_conversion_time: ConversionTime,

    pub shunt_voltage_conversion_time: ConversionTime,

    /// Период опроса устройства
    pub period: Duration,

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
            fn_init_requests,
            periodic_requests: vec![ConfigPeriodicRequest {
                period: self.period,
                fn_requests: |buffer| {
                    Ok(vec![FieldbusRequest::new(
                        buffer.config.address,
                        RequestKind::ReadMeasData,
                        vec![
                            INA226::read_shunt_voltage(),
                            INA226::read_bus_voltage(),
                            INA226::read_power(),
                            INA226::read_current(),
                        ],
                    )])
                },
            }],
            fn_msgs_to_buffer: |_, _| (),
            buffer_to_request_period: Duration::from_millis(1000),
            fn_buffer_to_request: |_| Ok(vec![]),
            fn_response_to_buffer: |response, buffer| {
                let kind: RequestKind = response.request_kind.try_into()?;

                let payload = match response.payload {
                    Ok(payload) => payload,
                    Err(err) => {
                        warn!("Error reading state: {}", err);
                        return ResponseResult::error(err);
                    }
                };

                match kind {
                    RequestKind::Init => {
                        buffer.read.manufacturer_id =
                            u16::from_be_bytes([payload[1][0], payload[1][1]]);

                        buffer.read.die_id = u16::from_be_bytes([payload[2][0], payload[2][1]]);

                        info!(
                            "INA226 from address {:?}: Manufacture ID = {:x}; Die ID = {:x}",
                            buffer.config.address, buffer.read.manufacturer_id, buffer.read.die_id,
                        );

                        ResponseResult::ok_init_completed()
                    }
                    RequestKind::ReadMeasData => {
                        let shunt_voltage = i16::from_be_bytes([payload[0][0], payload[0][1]]);
                        buffer.read.shunt_voltage =
                            shunt_voltage as f64 * buffer.config.lsb_shunt_voltage;

                        let bus_voltage = u16::from_be_bytes([payload[1][0], payload[1][1]]);
                        buffer.read.bus_voltage =
                            bus_voltage as f64 * buffer.config.lsb_bus_voltage;

                        let power = u16::from_be_bytes([payload[2][0], payload[2][1]]);
                        buffer.read.power = power as f64 * buffer.config.lsb_power;

                        let current = i16::from_be_bytes([payload[3][0], payload[3][1]]);
                        buffer.read.current = current as f64 * buffer.config.lsb_current;

                        ResponseResult::ok()
                    }
                }
            },
            fn_buffer_to_msgs: self.fn_output,
            buffer_default: Buffer {
                config: buffer::Config {
                    address: self.address,
                    average_mode: self.average_mode,
                    bus_voltage_conversion_time: self.bus_voltage_conversion_time,
                    shunt_voltage_conversion_time: self.shunt_voltage_conversion_time,
                    shunt_resistance: self.shunt_resistance,
                    lsb_shunt_voltage: 2.5e-6,
                    lsb_bus_voltage: 1.25e-3,
                    lsb_current: buffer::calc_current_lsb(self.max_current),
                    lsb_power: 25.0 * buffer::calc_current_lsb(self.max_current),
                },
                ..Default::default()
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
    vec![FieldbusRequest::new(
        buffer.config.address,
        RequestKind::Init,
        vec![
            INA226::write_config(ina226::ConfigRegister {
                reset: true,
                averaging: buffer.config.average_mode,
                bus_voltage_conversion_time: buffer.config.bus_voltage_conversion_time,
                shunt_voltage_conversion_time: buffer.config.shunt_voltage_conversion_time,
                operating_mode: ina226::OperatingMode::ShuntAndBusContinuous,
            }),
            INA226::read_man_id(),
            INA226::read_die_id(),
            INA226::write_calibration_register(calc_calibration_register(
                buffer.config.lsb_current,
                buffer.config.shunt_resistance,
            )),
        ],
    )]
}
