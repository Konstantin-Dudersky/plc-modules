use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::Peripherals};
use rsiot::{
    components::{cmp_esp_spi_master, cmp_esp_uart_slave, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
};
use tokio::task::LocalSet;

use pm_firmware_uart_shared::settings::MESSAGE_LEN;

use crate::{define_address, service::Service};

use super::Custom;

pub async fn main() -> anyhow::Result<()> {
    let peripherals = Peripherals::take().unwrap();

    // Периферия ESP -------------------------------------------------------------------------------
    let spi = peripherals.spi2;
    let mut pin_mosi: AnyIOPin = peripherals.pins.gpio4.into();
    let mut pin_miso: AnyIOPin = peripherals.pins.gpio5.into();
    let mut pin_sck: AnyIOPin = peripherals.pins.gpio6.into();
    let pin_cs_gpio_output = peripherals.pins.gpio8;

    let uart = peripherals.uart1;
    let pin_uart_rx = peripherals.pins.gpio20;
    let pin_uart_tx = peripherals.pins.gpio21;
    let pin_uart_rts = peripherals.pins.gpio10;

    // I2C address ---------------------------------------------------------------------------------
    let slave_address = define_address(20, &mut pin_mosi, &mut pin_miso, &mut pin_sck);

    // cmp_esp_uart_slave --------------------------------------------------------------------------
    let config_esp_uart_slave = super::config_esp_uart_slave::config::<_, _, MESSAGE_LEN>(
        slave_address,
        uart,
        pin_uart_rx.into(),
        pin_uart_tx.into(),
        pin_uart_rts.into(),
    );

    // cmp_esp_spi_master --------------------------------------------------------------------------
    let config_esp_spi_master = super::config_esp_spi_master::config(
        spi,
        pin_mosi,
        pin_miso,
        pin_sck,
        pin_cs_gpio_output.into(),
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
            .add_cmp(cmp_esp_uart_slave::Cmp::new(config_esp_uart_slave))
            .add_cmp(cmp_esp_spi_master::Cmp::new(config_esp_spi_master))
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;

    Ok(())
}
