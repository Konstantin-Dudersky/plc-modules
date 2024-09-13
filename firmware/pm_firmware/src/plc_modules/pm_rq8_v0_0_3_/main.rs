use std::time::Duration;

use esp_idf_svc::hal::prelude::Peripherals;
use rsiot::{
    components::{
        cmp_esp_i2c_slave::{self, BufferData},
        cmp_esp_spi_master,
    },
    message::Message,
};

use crate::define_address;

use super::{Custom, I2cRequest, I2cResponse};

pub async fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();

    // Периферия ESP -------------------------------------------------------------------------------
    let i2c = peripherals.i2c0;
    let spi = peripherals.spi2;

    let pin_a0 = peripherals.pins.gpio21;
    let pin_a1 = peripherals.pins.gpio20;
    let pin_a2 = peripherals.pins.gpio10;

    let pin_sda = peripherals.pins.gpio0;
    let pin_scl = peripherals.pins.gpio1;

    let pin_mosi = peripherals.pins.gpio2;
    let pin_miso = peripherals.pins.gpio3;
    let pin_sck = peripherals.pins.gpio9;
    let pin_cs_gpio_expander = peripherals.pins.gpio8;

    // I2C address ---------------------------------------------------------------------------------
    let slave_address = define_address(20, pin_a0.into(), pin_a1.into(), pin_a2.into());

    // cmp_esp_i2c_slave ---------------------------------------------------------------------------
    let config_esp_i2c_slave = cmp_esp_i2c_slave::Config {
        i2c,
        sda: pin_sda.into(),
        scl: pin_scl.into(),
        slave_address,
        fn_input: |_, _| (),
        fn_output: |buffer: &I2cBuffer| vec![Message::new_custom(Custom::SetOutput(buffer.output))],
        fn_output_period: Duration::from_millis(100),
        fn_i2c_comm: |req: I2cRequest, buffer: &mut I2cBuffer| match req {
            I2cRequest::SetOutputs(data) => {
                buffer.output = data;
                Ok(I2cResponse::Ok)
            }
        },
        buffer_data_default: I2cBuffer::default(),
    };

    // cmp_esp_spi_master --------------------------------------------------------------------------
    let config_esp_spi_master = cmp_esp_spi_master::Config {
        spi,
        pin_miso: pin_miso.into(),
        pin_mosi: pin_mosi.into(),
        pin_sck: pin_sck.into(),
        devices: vec![cmp_esp_spi_master::ConfigDevice {
            pin_cs: pin_cs_gpio_expander.into(),
            fn_init: todo!(),
            fn_input: todo!(),
            fn_output: todo!(),
            fn_output_period: todo!(),
        }],
    };

    Ok(())
}

#[derive(Clone, Debug, Default)]
struct I2cBuffer {
    pub output: u8,
}

impl BufferData for I2cBuffer {}
