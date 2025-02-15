use rsiot::components_config::spi_master::Operation;

pub struct AD7190 {}

impl AD7190 {
    pub fn read_status_register() -> Operation {
        Operation::WriteRead(vec![0b0100_0000], 1)
    }

    pub fn read_mode_register() -> Operation {
        Operation::WriteRead(vec![0b0_1_001_0_00], 3)
    }
}
