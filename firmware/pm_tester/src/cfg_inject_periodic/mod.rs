use std::time::Duration;

use rsiot::components::cmp_inject_periodic::*;

use crate::msg::*;

pub fn cmp_i2cdetect() -> Cmp<Msg, impl FnMut() -> Vec<Msg>> {
    let config = Config {
        period: Duration::from_millis(500),
        fn_periodic: move || {
            let msg = Msg::MsgInjPeriodic(MsgInjPeriodic::I2cDetect(()));
            vec![msg]
        },
    };

    Cmp::new(config)
}

pub fn cmp_watchdog() -> Cmp<Msg, impl FnMut() -> Vec<Msg>> {
    let mut watchdog = false;

    let config = Config {
        period: Duration::from_millis(500),
        fn_periodic: move || {
            let msg = Msg::MsgInjPeriodic(MsgInjPeriodic::Watchdog(watchdog));
            watchdog = !watchdog;
            vec![msg]
        },
    };

    Cmp::new(config)
}
