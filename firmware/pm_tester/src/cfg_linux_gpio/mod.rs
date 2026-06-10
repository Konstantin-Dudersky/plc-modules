use rsiot::components::cmp_linux_gpio::*;

use crate::msg::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        gpio_input: vec![],
        gpio_output: vec![ConfigGpioOutput {
            dev_gpio: "/dev/gpiochip0",
            gpio_line: 4,
            description: "WDT",
            fn_gpio_output: |msg| {
                if let Msg::MsgDerive(MsgDerive::Watchdog {
                    watchdog_signal, ..
                }) = msg
                {
                    return Some(watchdog_signal);
                }
                None
            },
            default_state: false,
        }],
    };

    Cmp::new(config)
}
