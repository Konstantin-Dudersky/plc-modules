use rsiot::components_config::{i2c_master::I2cAddress, master_device::BufferBound};

use super::OutputData;

/// Буфер данных
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    /// Адрес I2C устройства
    pub address: I2cAddress,

    /// Прочитанные данные
    pub read: Read,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub input_states: InputStates,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputStates {
    pub ch00: bool,
    pub ch01: bool,
    pub ch02: bool,
    pub ch03: bool,
    pub ch04: bool,
    pub ch05: bool,
    pub ch06: bool,
    pub ch07: bool,
    pub ch08: bool,
    pub ch09: bool,
    pub ch10: bool,
    pub ch11: bool,
    pub ch12: bool,
    pub ch13: bool,
    pub ch14: bool,
    pub ch15: bool,
}

impl BufferBound for Buffer {}

impl Buffer {
    /// Создать выходные данные
    pub fn output_data(&self) -> OutputData {
        OutputData {
            ch00: self.read.input_states.ch00,
            ch01: self.read.input_states.ch01,
            ch02: self.read.input_states.ch02,
            ch03: self.read.input_states.ch03,
            ch04: self.read.input_states.ch04,
            ch05: self.read.input_states.ch05,
            ch06: self.read.input_states.ch06,
            ch07: self.read.input_states.ch07,
            ch08: self.read.input_states.ch08,
            ch09: self.read.input_states.ch09,
            ch10: self.read.input_states.ch10,
            ch11: self.read.input_states.ch11,
            ch12: self.read.input_states.ch12,
            ch13: self.read.input_states.ch13,
            ch14: self.read.input_states.ch14,
            ch15: self.read.input_states.ch15,
        }
    }
}
