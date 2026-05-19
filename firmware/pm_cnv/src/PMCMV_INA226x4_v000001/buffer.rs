use rsiot::components_config::{i2c_master::I2cAddress, master_device::BufferBound};

use super::{AveragingMode, ConversionTime, OutputData};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub config: Config,

    pub write: Write,
    pub read: Read,
}
impl Buffer {
    pub fn output_data(&self) -> OutputData {
        OutputData {
            shunt_voltage: self.read.shunt_voltage,
            bus_voltage: self.read.bus_voltage,
            power: self.read.power,
            current: self.read.current,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Config {
    pub address: I2cAddress,
    pub average_mode: AveragingMode,
    pub bus_voltage_conversion_time: ConversionTime,
    pub shunt_voltage_conversion_time: ConversionTime,
    pub shunt_resistance: f64,
    pub lsb_shunt_voltage: f64,
    pub lsb_bus_voltage: f64,
    pub lsb_current: f64,
    pub lsb_power: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Write {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub shunt_voltage: f64,
    pub bus_voltage: f64,
    pub power: f64,
    pub current: f64,
    pub manufacturer_id: u16,
    pub die_id: u16,
}

impl BufferBound for Buffer {}

pub fn calc_current_lsb(max_current: f64) -> f64 {
    max_current / 32767.0
}

pub fn calc_calibration_register(current_lsb: f64, shunt_resistance: f64) -> u16 {
    let cal = 0.00512 / (current_lsb * shunt_resistance);
    cal.round() as u16
}

#[cfg(test)]
mod tests {}
