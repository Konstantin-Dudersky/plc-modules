use esp_idf_svc::hal::prelude::Peripherals;
use rsiot::{
    components::{cmp_esp_i2c_slave, cmp_esp_spi_master},
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
    let pin_cs_gpio_expander = peripherals.pins.gpio8;

    // I2C address ---------------------------------------------------------------------------------
    let slave_address = define_address(20, pin_a0.into(), pin_a1.into(), pin_a2.into());

    // cmp_esp_i2c_slave ---------------------------------------------------------------------------
    let config_esp_i2c_slave =
        super::config_esp_i2c_slave::config(i2c, pin_sda.into(), pin_scl.into(), slave_address);

    // cmp_esp_spi_master --------------------------------------------------------------------------
    let config_esp_spi_master = super::config_esp_spi_master::config(
        spi,
        pin_mosi.into(),
        pin_miso.into(),
        pin_sck.into(),
        pin_cs_gpio_expander.into(),
    );

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 50,
        service: Service::PM_RQ8,
        fn_auth: |msg, _| Some(msg),
    };

    let local_set = LocalSet::new();
    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_esp_i2c_slave::Cmp::new(config_esp_i2c_slave))
            .add_cmp(cmp_esp_spi_master::Cmp::new(config_esp_spi_master))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;

    Ok(())
}
