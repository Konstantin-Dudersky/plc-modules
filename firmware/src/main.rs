use std::time::Duration;

use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::*;
use esp_idf_svc::hal::{i2c, units::FromValueType};
use esp_idf_svc::hal::{
    prelude::Peripherals,
    spi::{SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let sclk = peripherals.pins.gpio9;
    let serial_in = peripherals.pins.gpio3;
    let serial_out = peripherals.pins.gpio2;
    let cs_1 = peripherals.pins.gpio8;

    let driver = SpiDriver::new::<SPI2>(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        &SpiDriverConfig::new(),
    )
    .unwrap();

    let config_1 = config::Config::new().baudrate(13.MHz().into());
    let mut device_1 = SpiDeviceDriver::new(&driver, Some(cs_1), &config_1).unwrap();

    let mut read = [0u8; 4];
    device_1
        .transaction(&mut [Operation::Write(&[0x41, 0x00]), Operation::Read(&mut read)])
        .unwrap();
    println!("Read: {:?}", read);

    let write_buf = [0x40, 0x00, 0x00];
    device_1.write(&write_buf).unwrap();

    let mut read = [0u8; 4];
    device_1
        .transaction(&mut [Operation::Write(&[0x41, 0x00]), Operation::Read(&mut read)])
        .unwrap();
    println!("Read: {:?}", read);

    let delay = Duration::from_millis(100);
    loop {
        let write_buf = [0x40, 0x12, 0xFF];
        device_1
            .transaction(&mut [Operation::Write(&write_buf)])
            .unwrap();

        sleep(delay).await;

        let write_buf = [0x40, 0x12, 0x00];
        device_1
            .transaction(&mut [Operation::Write(&write_buf)])
            .unwrap();

        sleep(delay).await;

        // device_1.write(&write_buf).unwrap();
        // println!("Result: {:?}", read_buf);
    }
}
