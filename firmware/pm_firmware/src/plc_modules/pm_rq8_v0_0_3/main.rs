use std::time::Duration;

use esp_idf_svc::hal::prelude::Peripherals;
use rsiot::{
    components::{cmp_esp_i2c_slave, cmp_esp_spi_master, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
};
use tokio::task::LocalSet;

use crate::{define_address, service::Service};

use super::Custom;

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
    let pin_cs_gpio_output = peripherals.pins.gpio8;
    let pin_cs_gpio_led = peripherals.pins.gpio7;

    // I2C address ---------------------------------------------------------------------------------
    let slave_address = define_address(
        20,
        &mut pin_a0.into(),
        &mut pin_a1.into(),
        &mut pin_a2.into(),
    );

    // cmp_esp_i2c_slave ---------------------------------------------------------------------------
    let config_esp_i2c_slave =
        super::config_esp_i2c_slave::config(i2c, pin_sda.into(), pin_scl.into(), slave_address);

    // cmp_esp_spi_master --------------------------------------------------------------------------
    let config_esp_spi_master = super::config_esp_spi_master::config(
        spi,
        pin_mosi.into(),
        pin_miso.into(),
        pin_sck.into(),
        pin_cs_gpio_output.into(),
        pin_cs_gpio_led.into(),
    );

    // cmp_logger ----------------------------------------------------------------------------------
    let config_logger = super::config_logger::config();

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 50,
        service: Service::PM_RQ8,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();
    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_esp_i2c_slave::Cmp::new(config_esp_i2c_slave))
            .add_cmp(cmp_esp_spi_master::Cmp::new(config_esp_spi_master))
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;

    Ok(())
}
