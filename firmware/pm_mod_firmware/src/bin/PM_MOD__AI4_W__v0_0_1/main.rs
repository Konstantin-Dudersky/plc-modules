use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::Peripherals};
use rsiot::{
    components::{cmp_esp_spi_master, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
};
use tokio::task::LocalSet;
use tracing::level_filters::LevelFilter;

use pm_mod_firmware::define_address;

mod config_derive;
mod config_esp_spi_master;
mod config_esp_uart_slave;
mod config_logger;
mod message;

use message::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    configure_logging(LevelFilter::INFO).unwrap();

    let peripherals = Peripherals::take().unwrap();

    // Периферия ESP -------------------------------------------------------------------------------
    let spi = peripherals.spi2;
    let mut pin_mosi: AnyIOPin = peripherals.pins.gpio0.into();
    let mut pin_miso: AnyIOPin = peripherals.pins.gpio1.into();
    let mut pin_sck: AnyIOPin = peripherals.pins.gpio3.into();
    let pin_cs0 = peripherals.pins.gpio10;
    let pin_cs1 = peripherals.pins.gpio2;

    let uart = peripherals.uart1;
    let pin_uart_rx = peripherals.pins.gpio20;
    let pin_uart_tx = peripherals.pins.gpio21;
    let pin_uart_rts = peripherals.pins.gpio4;

    // UART address --------------------------------------------------------------------------------
    // let uart_slave_address = define_address(&mut pin_mosi, &mut pin_miso, &mut pin_sck);
    let uart_slave_address = 0x0;

    // cmp_esp_spi_master --------------------------------------------------------------------------
    let config_esp_spi_master = config_esp_spi_master::config(
        spi,
        pin_mosi,
        pin_miso,
        pin_sck,
        pin_cs0.into(),
        pin_cs1.into(),
    );

    // cmp_logger ----------------------------------------------------------------------------------
    let config_logger = config_logger::config();

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 50,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let local_set = LocalSet::new();
    local_set.spawn_local(async move {
        ComponentExecutor::<MRoot>::new(executor_config)
            .add_cmp(config_esp_uart_slave::cmp(
                uart_slave_address,
                uart,
                pin_uart_rx.into(),
                pin_uart_tx.into(),
                pin_uart_rts.into(),
            ))
            .add_cmp(cmp_esp_spi_master::Cmp::new(config_esp_spi_master))
            .add_cmp(cmp_logger::Cmp::new(config_logger))
            .add_cmp(config_derive::new())
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;

    Ok(())
}
