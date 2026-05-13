use bitvec::{order::Msb0, view::BitView};
use rsiot::components_config::i2c_master::Operation;

const REG_CONFIG: u8 = 0x00;
const REG_SHUNT_VOLTAGE: u8 = 0x01;
const REG_BUS_VOLTAGE: u8 = 0x02;
const REG_POWER: u8 = 0x03;
const REG_CURRENT: u8 = 0x04;
const REG_CALIBRATION: u8 = 0x05;
const REG_MAN_ID: u8 = 0xFE;
const REG_DIE_ID: u8 = 0xFF;

pub struct INA226 {}

impl INA226 {
    pub fn write_config(config: ConfigRegister) -> Operation {
        let config_encode = config.encode();

        Operation::Write {
            write_data: vec![REG_CONFIG, config_encode[0], config_encode[1]],
        }
    }

    pub fn read_man_id() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_MAN_ID],
            read_size: 2,
        }
    }

    pub fn read_die_id() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_DIE_ID],
            read_size: 2,
        }
    }

    pub fn write_calibration_register(cal: u16) -> Operation {
        let cal_bytes = cal.to_be_bytes();

        Operation::Write {
            write_data: vec![REG_CALIBRATION, cal_bytes[0], cal_bytes[1]],
        }
    }

    pub fn read_shunt_voltage() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_SHUNT_VOLTAGE],
            read_size: 2,
        }
    }

    pub fn read_bus_voltage() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_BUS_VOLTAGE],
            read_size: 2,
        }
    }

    pub fn read_power() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_POWER],
            read_size: 2,
        }
    }

    pub fn read_current() -> Operation {
        Operation::WriteRead {
            write_data: vec![REG_CURRENT],
            read_size: 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigRegister {
    pub reset: bool,
    pub averaging: AveragingMode,
    pub bus_voltage_conversion_time: ConversionTime,
    pub shunt_voltage_conversion_time: ConversionTime,
    pub operating_mode: OperatingMode,
}
impl ConfigRegister {
    pub fn encode(&self) -> [u8; 2] {
        let mut bytes = [0; 2];
        let bits = bytes.view_bits_mut::<Msb0>();

        bits.set(0, self.reset);

        let [avg2, avg1, avg0] = self.averaging.encode();
        bits.set(4, avg2);
        bits.set(5, avg1);
        bits.set(6, avg0);

        let [vbusct2, vbusct1, vbusct0] = self.bus_voltage_conversion_time.encode();
        bits.set(7, vbusct2);
        bits.set(8, vbusct1);
        bits.set(9, vbusct0);

        let [vshct2, vshct1, vshct0] = self.shunt_voltage_conversion_time.encode();
        bits.set(10, vshct2);
        bits.set(11, vshct1);
        bits.set(12, vshct0);

        let [mode3, mode2, mode1] = self.operating_mode.encode();
        bits.set(13, mode3);
        bits.set(14, mode2);
        bits.set(15, mode1);

        bytes
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AveragingMode {
    #[default]
    _1,
    _4,
    _16,
    _64,
    _128,
    _256,
    _512,
    _1024,
}
impl AveragingMode {
    pub fn encode(&self) -> [bool; 3] {
        match self {
            Self::_1 => [false, false, false],
            Self::_4 => [false, false, true],
            Self::_16 => [false, true, false],
            Self::_64 => [false, true, true],
            Self::_128 => [true, false, false],
            Self::_256 => [true, false, true],
            Self::_512 => [true, true, false],
            Self::_1024 => [true, true, true],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ConversionTime {
    _140us,
    _204us,
    _332us,
    _588us,
    #[default]
    _1100us,
    _2116us,
    _4156us,
    _8244us,
}
impl ConversionTime {
    pub fn encode(&self) -> [bool; 3] {
        match self {
            Self::_140us => [false, false, false],
            Self::_204us => [false, false, true],
            Self::_332us => [false, true, false],
            Self::_588us => [false, true, true],
            Self::_1100us => [true, false, false],
            Self::_2116us => [true, false, true],
            Self::_4156us => [true, true, false],
            Self::_8244us => [true, true, true],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum OperatingMode {
    Shutdown,
    ShuntTriggered,
    BusTriggered,
    ShuntAndBusTriggered,
    ShuntContinuous,
    BusContinuous,
    #[default]
    ShuntAndBusContinuous,
}
impl OperatingMode {
    pub fn encode(&self) -> [bool; 3] {
        match self {
            Self::Shutdown => [false, false, false],
            Self::ShuntTriggered => [false, false, true],
            Self::BusTriggered => [false, true, false],
            Self::ShuntAndBusTriggered => [false, true, true],
            Self::ShuntContinuous => [true, false, true],
            Self::BusContinuous => [true, true, false],
            Self::ShuntAndBusContinuous => [true, true, true],
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn config_register_default() {
        let cr = ConfigRegister {
            reset: false,
            averaging: AveragingMode::_1,
            bus_voltage_conversion_time: ConversionTime::_1100us,
            shunt_voltage_conversion_time: ConversionTime::_1100us,
            operating_mode: OperatingMode::ShuntAndBusContinuous,
        };

        assert_eq!(cr.encode(), [0x01, 0x27])
    }
}
