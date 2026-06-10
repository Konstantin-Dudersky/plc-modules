use rsiot::{components::cmp_logger::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::InputDia0(v) => format!("a.0: {}", v),
                Msg::InputDia1(v) => format!("a.1: {}", v),
                Msg::InputDia2(v) => format!("a.2: {}", v),
                Msg::InputDia3(v) => format!("a.3: {}", v),
                Msg::InputDia4(v) => format!("a.4: {}", v),
                Msg::InputDia5(v) => format!("a.5: {}", v),
                Msg::InputDia6(v) => format!("a.6: {}", v),
                Msg::InputDia7(v) => format!("a.7: {}", v),
                Msg::InputDib0(v) => format!("b.0: {}", v),
                Msg::InputDib1(v) => format!("b.1: {}", v),
                Msg::InputDib2(v) => format!("b.2: {}", v),
                Msg::InputDib3(v) => format!("b.3: {}", v),
                Msg::InputDib4(v) => format!("b.4: {}", v),
                Msg::InputDib5(v) => format!("b.5: {}", v),
                Msg::InputDib6(v) => format!("b.6: {}", v),
                Msg::InputDib7(v) => format!("b.7: {}", v),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
