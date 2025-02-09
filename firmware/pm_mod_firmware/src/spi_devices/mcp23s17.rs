use esp_idf_svc::hal::spi::{Operation, SpiDeviceDriver, SpiDriver};

const WRITE_ADDR: u8 = 0x40;
const READ_ADDR: u8 = 0x41;

const REG_IODIRA: u8 = 0x00;
const REG_IODIRB: u8 = 0x01;
const REG_IPOLA: u8 = 0x02;
const REG_IPOLB: u8 = 0x03;
const REG_GPINTENA: u8 = 0x04;
const REG_GPINTENB: u8 = 0x05;
// const REG_DEFVALA: u8 = 0x06;
// const REG_DEFVALB: u8 = 0x07;
// const REG_INTCONA: u8 = 0x08;
// const REG_INTCONB: u8 = 0x09;
const REG_IOCON: u8 = 0x0A;
// const REG_GPPUA: u8 = 0x0C;
// const REG_GPPUB: u8 = 0x0D;
const REG_INTFA: u8 = 0x0E;
const REG_INTFB: u8 = 0x0F;
// const REG_INTCAPA: u8 = 0x10;
// const REG_INTCAPB: u8 = 0x11;
const REG_GPIOA: u8 = 0x12;
const REG_GPIOB: u8 = 0x13;
// const REG_OLATA: u8 = 0x14;
// const REG_OLATB: u8 = 0x15;

pub struct MCP23S17 {}

#[allow(dead_code)]
impl MCP23S17 {
    /// Установить режим работы как вход или как выход:
    /// - 0x00 - выходы
    /// - 0xFF - входы
    pub fn iodir_a_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IODIRA, value];
        device_driver.write(&buffer).unwrap()
    }

    /// Установить режим работы как вход или как выход:
    /// - 0x00 - выходы
    /// - 0xFF - входы
    pub fn iodir_b_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IODIRB, value];
        device_driver.write(&buffer).unwrap()
    }

    /// Полярность входов:
    /// - 0x00 - прямая полярность
    /// - 0xFF - обратная полярность
    pub fn ipol_a_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IPOLA, value];
        device_driver.write(&buffer).unwrap()
    }

    /// Полярность входов:
    /// - 0x00 - прямая полярность
    /// - 0xFF - обратная полярность
    pub fn ipol_b_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IPOLB, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn gpinten_a_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_GPINTENA, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn gpinten_b_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_GPINTENB, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn iocon_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_IOCON, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn iocon_get<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_IOCON];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn intf_a_get<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_INTFA];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn intf_b_get<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_INTFB];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn gpio_a_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_GPIOA, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn gpio_a_get<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_GPIOA];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }

    pub fn gpio_b_set<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>, value: u8) {
        let buffer = [WRITE_ADDR, REG_GPIOB, value];
        device_driver.write(&buffer).unwrap()
    }

    pub fn gpio_b_get<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> u8 {
        let buffer = [READ_ADDR, REG_GPIOB];
        let mut read_buf = [0; 1];
        device_driver
            .transaction(&mut [Operation::Write(&buffer), Operation::Read(&mut read_buf)])
            .unwrap();
        read_buf[0]
    }
}
