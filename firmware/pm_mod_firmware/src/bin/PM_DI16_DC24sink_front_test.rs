//! cargo run --release --bin PM_DI16_DC24sink_front_test

#![allow(non_snake_case)]

use esp_idf_svc::hal::{
    prelude::Peripherals,
    spi::{config, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::FromValueType,
};

use pm_mod_firmware::spi_devices::mcp23s17::MCP23S17;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    let peripherals = Peripherals::take().unwrap();

    let spi = peripherals.spi2;

    let pin_mosi = peripherals.pins.gpio2;
    let pin_miso = peripherals.pins.gpio3;
    let pin_sck = peripherals.pins.gpio9;
    let pin_cs = peripherals.pins.gpio8;

    let spi_master_driver = SpiDriver::new(
        spi,
        pin_sck,
        pin_mosi,
        Some(pin_miso),
        &SpiDriverConfig::new(),
    )
    .unwrap();

    let config = config::Config::new().baudrate(13.MHz().into());
    let mut spi_slave = SpiDeviceDriver::new(&spi_master_driver, Some(pin_cs), &config).unwrap();

    loop {
        // Задаем gpio как выходы
        MCP23S17::iodir_a_set(&mut spi_slave, 0x00);
        MCP23S17::iodir_b_set(&mut spi_slave, 0x00);
        // Устанавливаем все выходы
        MCP23S17::gpio_a_set(&mut spi_slave, 0xFF);
        MCP23S17::gpio_b_set(&mut spi_slave, 0xFF);
    }
}
