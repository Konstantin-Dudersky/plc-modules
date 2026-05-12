use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;

    let config = Config {
        period: Duration::from_millis(500),
        fn_periodic: move || {
            let msgs = match counter {
                0..5 => vec![
                    Msg::OutputRq0(true),
                    Msg::OutputRq1(true),
                    Msg::OutputRq2(true),
                    Msg::OutputRq3(true),
                    Msg::OutputRq4(true),
                    Msg::OutputRq5(true),
                    Msg::OutputRq6(true),
                    Msg::OutputRq7(true),
                ],
                5..10 => vec![
                    Msg::OutputRq0(false),
                    Msg::OutputRq1(false),
                    Msg::OutputRq2(false),
                    Msg::OutputRq3(false),
                    Msg::OutputRq4(false),
                    Msg::OutputRq5(false),
                    Msg::OutputRq6(false),
                    Msg::OutputRq7(false),
                ],
                10 => vec![Msg::OutputRq0(true), Msg::OutputRq7(false)],
                11 => vec![Msg::OutputRq1(true), Msg::OutputRq0(false)],
                12 => vec![Msg::OutputRq2(true), Msg::OutputRq1(false)],
                13 => vec![Msg::OutputRq3(true), Msg::OutputRq2(false)],
                14 => vec![Msg::OutputRq4(true), Msg::OutputRq3(false)],
                15 => vec![Msg::OutputRq5(true), Msg::OutputRq4(false)],
                16 => vec![Msg::OutputRq6(true), Msg::OutputRq5(false)],
                17 => vec![Msg::OutputRq7(true), Msg::OutputRq6(false)],
                _ => panic!("Error"),
            };
            counter += 1;
            if counter > 17 {
                counter = 0;
            }

            msgs
        },
    };

    Cmp::new(config)
}
