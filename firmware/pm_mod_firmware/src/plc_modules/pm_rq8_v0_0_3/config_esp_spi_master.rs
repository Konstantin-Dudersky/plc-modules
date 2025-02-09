use std::{thread::sleep, time::Duration};

use esp_idf_svc::hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    spi::{Spi, SpiDeviceDriver, SpiDriver},
};
use rsiot::{components::cmp_esp_spi_master, message::Message};
use tracing::info;

use crate::{settings::DEBUG, spi_devices::mcp23s17::MCP23S17};

use super::Custom;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs_gpio_output: AnyIOPin,
    pin_cs_gpio_led: AnyIOPin,
) -> cmp_esp_spi_master::Config<Custom, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    let gpio_relay = cmp_esp_spi_master::ConfigDevice {
        pin_cs: pin_cs_gpio_output,
        fn_init: gpio_relay_fn_init,
        fn_input: gpio_relay_fn_input,
        fn_output: |_| vec![],
    };

    let gpio_led = cmp_esp_spi_master::ConfigDevice {
        pin_cs: pin_cs_gpio_led,
        fn_init: gpio_led_fn_init,
        fn_input: gpio_led_fn_input,
        fn_output: gpio_led_fn_output,
    };

    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices: vec![gpio_relay, gpio_led],
        fn_output_period: Duration::from_millis(100),
    }
}

fn gpio_relay_fn_init<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) {
    MCP23S17::iodir_a_set(device_driver, 0x00);
    if DEBUG {
        MCP23S17::gpio_a_set(device_driver, 0x00);
    }
    info!("MCP23S17 for relay inited");
}

fn gpio_relay_fn_input<'a>(
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

fn gpio_led_fn_init<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) {
    MCP23S17::iodir_a_set(device_driver, 0x00);

    MCP23S17::gpio_a_set(device_driver, 0x55);
    sleep(Duration::from_millis(200));
    MCP23S17::gpio_a_set(device_driver, 0xAA);
    sleep(Duration::from_millis(200));
    MCP23S17::gpio_a_set(device_driver, 0x55);
    sleep(Duration::from_millis(200));
    MCP23S17::gpio_a_set(device_driver, 0xAA);
    sleep(Duration::from_millis(200));

    MCP23S17::gpio_a_set(device_driver, 0x00);
    info!("MCP23S17 for led inited");
}

fn gpio_led_fn_input<'a>(
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

fn gpio_led_fn_output<'a>(
    _device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) -> Vec<Message<Custom>> {
    vec![]
}
