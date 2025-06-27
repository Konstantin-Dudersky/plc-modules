use rsiot::{components::cmp_logger::*, executor::Component};

use super::messages::*;

pub fn cmp() -> Component<Config<MRoot>, MRoot> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                MRoot::InputDia0(v) => format!("a.0: {}", v),
                MRoot::InputDia1(v) => format!("a.1: {}", v),
                MRoot::InputDia2(v) => format!("a.2: {}", v),
                MRoot::InputDia3(v) => format!("a.3: {}", v),
                MRoot::InputDia4(v) => format!("a.4: {}", v),
                MRoot::InputDia5(v) => format!("a.5: {}", v),
                MRoot::InputDia6(v) => format!("a.6: {}", v),
                MRoot::InputDia7(v) => format!("a.7: {}", v),
                MRoot::InputDib0(v) => format!("b.0: {}", v),
                MRoot::InputDib1(v) => format!("b.1: {}", v),
                MRoot::InputDib2(v) => format!("b.2: {}", v),
                MRoot::InputDib3(v) => format!("b.3: {}", v),
                MRoot::InputDib4(v) => format!("b.4: {}", v),
                MRoot::InputDib5(v) => format!("b.5: {}", v),
                MRoot::InputDib6(v) => format!("b.6: {}", v),
                MRoot::InputDib7(v) => format!("b.7: {}", v),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
