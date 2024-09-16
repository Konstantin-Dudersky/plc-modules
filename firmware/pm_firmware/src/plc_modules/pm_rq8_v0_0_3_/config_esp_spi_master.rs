use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use crate::spi_devices::mcp23s17_::MCP23S17;

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
    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices: vec![cmp_esp_spi_master::ConfigDevice {
            pin_cs: pin_cs_gpio_expander,
            fn_init: |device_driver| MCP23S17::set_iodira(device_driver, 0x00),
            fn_input: |msg: &Message<Custom>, device_driver| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                match msg {
                    Custom::SetOutput(data) => MCP23S17::set_gpioa(device_driver, data),
                }
            },
            fn_output: |_| vec![],
        }],
        fn_output_period: Duration::from_millis(1000),
    }
}
