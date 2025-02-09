use esp_idf_svc::hal::gpio::AnyIOPin;
use rsiot::{components::cmp_esp_gpio, message::Message};

use super::Custom;

pub fn config(pin_int_gpio_expander: AnyIOPin) -> cmp_esp_gpio::Config<Custom> {
    cmp_esp_gpio::Config {
        inputs: vec![cmp_esp_gpio::ConfigGpioInput {
            peripherals: pin_int_gpio_expander,
            fn_output: |value| {
                let msg = Message::new_custom(Custom::GpioExpanderInt(value));
                msg
            },
            pull: cmp_esp_gpio::Pull::Down,
        }],
        outputs: vec![],
    }
}
