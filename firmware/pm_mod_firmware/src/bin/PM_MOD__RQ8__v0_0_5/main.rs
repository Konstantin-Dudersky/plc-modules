mod config_esp_spi_master;
// mod config_esp_uart_slave;
mod config_livecounter;
mod config_logger;
mod message;

use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::Peripherals};
use rsiot::{
    components::{cmp_esp_spi_master, cmp_esp_uart_slave, cmp_livecounter, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
};
use tokio::task::LocalSet;

use pm_mod_firmware::define_address;

use message::Custom;
use tracing::level_filters::LevelFilter;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    configure_logging(LevelFilter::INFO).unwrap();

    let peripherals = Peripherals::take().unwrap();

    // Периферия ESP -------------------------------------------------------------------------------
    let spi = peripherals.spi2;
    let mut pin_mosi: AnyIOPin = peripherals.pins.gpio0.into();
    let mut pin_miso: AnyIOPin = peripherals.pins.gpio1.into();
    let mut pin_sck: AnyIOPin = peripherals.pins.gpio3.into();
    let pin_cs0 = peripherals.pins.gpio10;

    let uart = peripherals.uart1;
    let pin_uart_rx = peripherals.pins.gpio20;
    let pin_uart_tx = peripherals.pins.gpio21;
    let pin_uart_rts = peripherals.pins.gpio4;

    // UART address --------------------------------------------------------------------------------
    // let uart_slave_address = define_address(0, &mut pin_sck, &mut pin_miso, &mut pin_mosi);
    let uart_slave_address = 1;
    // Конфигурация компонентов --------------------------------------------------------------------
    // let config_esp_uart_slave = config_esp_uart_slave::config::<_, _>(
    //     uart_slave_address,
    //     uart,
    //     pin_uart_rx.into(),
    //     pin_uart_tx.into(),
    //     pin_uart_rts.into(),
    // );

    let config_esp_spi_master =
        config_esp_spi_master::config(spi, pin_mosi, pin_miso, pin_sck, pin_cs0.into());

    let config_logger = config_logger::config();

    let config_livecounter = config_livecounter::config();

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 50,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();
    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            // .add_cmp(cmp_esp_uart_slave::Cmp::new(config_esp_uart_slave))
            .add_cmp(cmp_esp_spi_master::Cmp::new(config_esp_spi_master))
            .add_cmp(cmp_livecounter::Cmp::new(config_livecounter))
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;

    Ok(())
}
