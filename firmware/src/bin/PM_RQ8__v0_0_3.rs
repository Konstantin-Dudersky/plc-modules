use esp_idf_svc::hal::{
    i2c::{I2cSlaveConfig, I2cSlaveDriver},
    prelude::Peripherals,
    spi::{SpiDriver, SpiDriverConfig, SPI2},
};

use pm_firmware::plc_modules::pm_rq8_v0_0_3;

const BUFFER_LEN: usize = 128;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // SPI -----------------------------------------------------------------------------------------

    let spi = peripherals.spi2;

    let pin_mosi = peripherals.pins.gpio2;
    let pin_miso = peripherals.pins.gpio3;
    let pin_cs_gpio_expander = peripherals.pins.gpio8;
    let pin_sck = peripherals.pins.gpio9;

    let spi_master_driver = SpiDriver::new::<SPI2>(
        spi,
        pin_sck,
        pin_mosi,
        Some(pin_miso),
        &SpiDriverConfig::new(),
    )
    .unwrap();

    // I2C -----------------------------------------------------------------------------------------

    let i2c = peripherals.i2c0;

    let pin_sda = peripherals.pins.gpio0;
    let pin_scl = peripherals.pins.gpio1;

    // TODO - сделать через DIP
    let slave_address = 0x02;

    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(BUFFER_LEN)
        .rx_buffer_length(BUFFER_LEN);

    let i2c_slave_driver =
        I2cSlaveDriver::new(i2c, pin_sda, pin_scl, slave_address, &i2c_idf_config).unwrap();

    // module --------------------------------------------------------------------------------------

    let module = pm_rq8_v0_0_3::Module {
        spi_master_driver: &spi_master_driver,
        pin_cs_gpio_expander: pin_cs_gpio_expander.into(),
        i2c_slave_driver,
    };

    module.spawn().await;
}
