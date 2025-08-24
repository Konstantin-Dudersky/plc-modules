use std::time::Duration;

use rsiot::{components::cmp_inject_periodic::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg, impl FnMut() -> Vec<Msg>>, Msg> {
    let mut counter = 0;

    let config = Config {
        period: Duration::from_millis(200),
        fn_periodic: move || {
            let msgs = match counter {
                0..5 => vec![
                    Msg::OutputDia0(true),
                    Msg::OutputDia1(true),
                    Msg::OutputDia2(true),
                    Msg::OutputDia3(true),
                    Msg::OutputDia4(true),
                    Msg::OutputDia5(true),
                    Msg::OutputDia6(true),
                    Msg::OutputDia7(true),
                    Msg::OutputDib0(true),
                    Msg::OutputDib1(true),
                    Msg::OutputDib2(true),
                    Msg::OutputDib3(true),
                    Msg::OutputDib4(true),
                    Msg::OutputDib5(true),
                    Msg::OutputDib6(true),
                    Msg::OutputDib7(true),
                ],
                5..10 => vec![
                    Msg::OutputDia0(false),
                    Msg::OutputDia1(false),
                    Msg::OutputDia2(false),
                    Msg::OutputDia3(false),
                    Msg::OutputDia4(false),
                    Msg::OutputDia5(false),
                    Msg::OutputDia6(false),
                    Msg::OutputDia7(false),
                    Msg::OutputDib0(false),
                    Msg::OutputDib1(false),
                    Msg::OutputDib2(false),
                    Msg::OutputDib3(false),
                    Msg::OutputDib4(false),
                    Msg::OutputDib5(false),
                    Msg::OutputDib6(false),
                    Msg::OutputDib7(false),
                ],
                10 => vec![Msg::OutputDia0(true), Msg::OutputDib7(false)],
                11 => vec![Msg::OutputDia1(true), Msg::OutputDia0(false)],
                12 => vec![Msg::OutputDia2(true), Msg::OutputDia1(false)],
                13 => vec![Msg::OutputDia3(true), Msg::OutputDia2(false)],
                14 => vec![Msg::OutputDia4(true), Msg::OutputDia3(false)],
                15 => vec![Msg::OutputDia5(true), Msg::OutputDia4(false)],
                16 => vec![Msg::OutputDia6(true), Msg::OutputDia5(false)],
                17 => vec![Msg::OutputDia7(true), Msg::OutputDia6(false)],
                18 => vec![Msg::OutputDib0(true), Msg::OutputDia7(false)],
                19 => vec![Msg::OutputDib1(true), Msg::OutputDib0(false)],
                20 => vec![Msg::OutputDib2(true), Msg::OutputDib1(false)],
                21 => vec![Msg::OutputDib3(true), Msg::OutputDib2(false)],
                22 => vec![Msg::OutputDib4(true), Msg::OutputDib3(false)],
                23 => vec![Msg::OutputDib5(true), Msg::OutputDib4(false)],
                24 => vec![Msg::OutputDib6(true), Msg::OutputDib5(false)],
                25 => vec![Msg::OutputDib7(true), Msg::OutputDib6(false)],
                _ => panic!("Error"),
            };
            counter += 1;
            if counter > 25 {
                counter = 0;
            }
            msgs
        },
    };

    Cmp::new(config)
}
