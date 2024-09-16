use std::time::Duration;

use esp_idf_svc::hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    spi::{Spi, SpiDeviceDriver, SpiDriver},
};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use crate::spi_devices::mcp23s17::MCP23S17;

use super::Custom;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs_gpio_expander: AnyIOPin,
) -> cmp_esp_spi_master::Config<Custom, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    let device_gpio_expander = cmp_esp_spi_master::ConfigDevice {
        pin_cs: pin_cs_gpio_expander,
        fn_init: gpio_expander_fn_init,
        fn_input: gpio_expander_fn_input,
        fn_output: |_| vec![],
    };

    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices: vec![device_gpio_expander],
        fn_output_period: Duration::from_millis(1000),
    }
}

fn gpio_expander_fn_init<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) {
    MCP23S17::iodir_a_set(device_driver, 0x00);
}

fn gpio_expander_fn_input<'a>(
    msg: &Message<Custom>,
    device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) {
    let Some(msg) = msg.get_custom_data() else {
        return;
    };
    match msg {
        Custom::SetOutput(data) => MCP23S17::gpio_a_set(device_driver, data),
    }
}
