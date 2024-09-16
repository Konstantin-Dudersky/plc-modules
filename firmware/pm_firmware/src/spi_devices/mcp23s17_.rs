use esp_idf_svc::hal::spi::{Operation, SpiDeviceDriver, SpiDriver};

const WRITE_ADDR: u8 = 0x40;
const READ_ADDR: u8 = 0x41;

const REG_IODIRA: u8 = 0x00;
const REG_IODIRB: u8 = 0x01;
// const REG_IPOLA: u8 = 0x02;
// const REG_IPOLB: u8 = 0x03;
// const REG_GPINTENA: u8 = 0x04;
// const REG_GPINTENB: u8 = 0x05;
// const REG_DEFVALA: u8 = 0x06;
// const REG_DEFVALB: u8 = 0x07;
// const REG_INTCONA: u8 = 0x08;
// const REG_INTCONB: u8 = 0x09;
// const REG_IOCONA: u8 = 0x0A;
// const REG_IOCONB: u8 = 0x0B;
// const REG_GPPUA: u8 = 0x0C;
// const REG_GPPUB: u8 = 0x0D;
// const REG_INTFA: u8 = 0x0E;
// const REG_INTFB: u8 = 0x0F;
// const REG_INTCAPA: u8 = 0x10;
// const REG_INTCAPB: u8 = 0x11;
const REG_GPIOA: u8 = 0x12;
const REG_GPIOB: u8 = 0x13;
// const REG_OLATA: u8 = 0x14;
// const REG_OLATB: u8 = 0x15;

pub struct MCP23S17 {}

impl MCP23S17 {
    pub fn set_iodira<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IODIRA, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn get_iodira<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_IODIRA];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn set_iodirb<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IODIRB, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn set_gpioa<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_GPIOA, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn get_gpioa<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_GPIOA];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn get_gpiob<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_GPIOB];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }
}
