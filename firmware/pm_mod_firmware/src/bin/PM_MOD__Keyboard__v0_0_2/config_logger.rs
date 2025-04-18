use rsiot::{
    components::cmp_logger::{self, *},
    executor::Component,
};

use super::message::*;

pub fn cmp() -> Component<Config<MRoot>, MRoot> {
    let config = cmp_logger::Config {
        level: cmp_logger::Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                MRoot::SpiMaster(MSpiMaster::PressedButton(b)) => {
                    format!("Pressed button: {:?}", b)
                }
                MRoot::SpiMaster(MSpiMaster::PressedTouch(t)) => {
                    format!("Pressed touch: {:?}", t)
                }
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
