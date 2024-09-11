use esp_idf_svc::hal::{
    i2c::{I2cSlaveConfig, I2cSlaveDriver},
    prelude::Peripherals,
    spi::{SpiDriver, SpiDriverConfig, SPI2},
};

use pm_firmware::{define_address, plc_modules::pm_di16_dc24sink_V0_0_2};

const BUFFER_LEN: usize = 128;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    let peripherals = Peripherals::take().unwrap();

    // I2C address ---------------------------------------------------------------------------------
    let pin_a0 = peripherals.pins.gpio21;
    let pin_a1 = peripherals.pins.gpio20;
    let pin_a2 = peripherals.pins.gpio10;
    let slave_address = define_address(10, pin_a0.into(), pin_a1.into(), pin_a2.into());

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

    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(BUFFER_LEN)
        .rx_buffer_length(BUFFER_LEN);

    let i2c_slave_driver =
        I2cSlaveDriver::new(i2c, pin_sda, pin_scl, slave_address, &i2c_idf_config).unwrap();

    // module --------------------------------------------------------------------------------------
    let module = pm_di16_dc24sink_V0_0_2::Module {
        spi_master_driver: &spi_master_driver,
        pin_cs_gpio_expander: pin_cs_gpio_expander.into(),
        i2c_slave_driver,
    };

    module.spawn().await;
}
