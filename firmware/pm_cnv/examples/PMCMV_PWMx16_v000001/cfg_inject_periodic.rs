use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use super::msg::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0.0;

    let config = Config {
        period: Duration::from_millis(100),
        fn_periodic: move || {
            let msgs = vec![Msg::Ch15(10.0)];

            counter += 1.0;
            if counter >= 100.0 {
                counter = 0.0;
            }

            msgs
        },
    };

    Cmp::new(config)
}
