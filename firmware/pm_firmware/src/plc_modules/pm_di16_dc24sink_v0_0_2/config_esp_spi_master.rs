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
    pin_cs_inputs: AnyIOPin,
    pin_cs_led: AnyIOPin,
) -> cmp_esp_spi_master::Config<Custom, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    let device_gpio_input = cmp_esp_spi_master::ConfigDevice {
        pin_cs: pin_cs_inputs,
        fn_init: gpio_input_fn_init,
        fn_input: gpio_input_fn_input,
        fn_output: gpio_input_fn_output,
    };

    let device_gpio_led = cmp_esp_spi_master::ConfigDevice {
        pin_cs: pin_cs_led,
        fn_init: gpio_led_fn_init,
        fn_input: gpio_led_fn_input,
        fn_output: gpio_led_fn_output,
    };

    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices: vec![device_gpio_input, device_gpio_led],
        fn_output_period: Duration::from_millis(100),
    }
}

fn gpio_input_fn_init<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) {
    // Задаем gpio как входы
    MCP23S17::iodir_a_set(device_driver, 0xFF);
    MCP23S17::iodir_b_set(device_driver, 0xFF);
    // Полярность входов
    MCP23S17::ipol_a_set(device_driver, 0x00);
    MCP23S17::ipol_b_set(device_driver, 0x00);
    // Настраиваем прерывание по изменению
    MCP23S17::gpinten_a_set(device_driver, 0xFF);
    MCP23S17::gpinten_b_set(device_driver, 0xFF);
    // Конфигурация
    MCP23S17::iocon_set(device_driver, 0b0100_0010);
}

fn gpio_input_fn_input<'a>(
    _msg: &Message<Custom>,
    _device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) {
}

fn gpio_input_fn_output<'a>(
    device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) -> Vec<Message<Custom>> {
    let gpioa = MCP23S17::gpio_a_get(device_driver);
    let gpiob = MCP23S17::gpio_b_get(device_driver);

    let msg = Message::new_custom(Custom::InputsState { a: gpioa, b: gpiob });

    vec![msg]
}

fn gpio_led_fn_init<'a>(device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>) {
    // Задаем gpio как выходы
    MCP23S17::iodir_a_set(device_driver, 0x00);
    MCP23S17::iodir_b_set(device_driver, 0x00);
    // TODO - удалить
    MCP23S17::gpio_a_set(device_driver, 0x00);
}

fn gpio_led_fn_input<'a>(
    msg: &Message<Custom>,
    device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) {
    let Some(msg) = msg.get_custom_data() else {
        return;
    };

    match msg {
        Custom::LedState { a, b } => {
            MCP23S17::gpio_a_set(device_driver, a);
            MCP23S17::gpio_b_set(device_driver, b);
        }
        _ => return,
    }
}

fn gpio_led_fn_output<'a>(
    _device_driver: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
) -> Vec<Message<Custom>> {
    vec![]
}
