use rsiot::components_config::i2c_master::Operation;

const REG_MODE1: u8 = 0x00;
const REG_MODE2: u8 = 0x01;
const REG_LED0_ON_L: u8 = 0x06;
const REG_PRESCALE: u8 = 0xFE;

pub struct PCA9685 {}

impl PCA9685 {
    pub fn write_mode1(mode1: u8) -> Operation {
        Operation::Write {
            write_data: vec![REG_MODE1, mode1],
        }
    }

    pub fn write_mode2(mode2: u8) -> Operation {
        Operation::Write {
            write_data: vec![REG_MODE2, mode2],
        }
    }

    pub fn write_led0_15(v: Vec<u8>) -> Result<Operation, String> {
        if v.len() != 64 {
            return Err("v must be 64 bytes".to_string());
        }

        let mut data = vec![REG_LED0_ON_L];
        data.extend_from_slice(&v);

        Ok(Operation::Write { write_data: data })
    }

    pub fn write_prescale(prescale: u8) -> Operation {
        Operation::Write {
            write_data: vec![REG_PRESCALE, prescale],
        }
    }
}
