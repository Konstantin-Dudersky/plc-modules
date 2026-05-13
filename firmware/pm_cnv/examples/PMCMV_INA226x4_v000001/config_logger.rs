use rsiot::components::cmp_logger::*;

use crate::messages::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                Msg::Ch0MeasuredValues(data) => {
                    format!("CH0 measured data: {}", data)
                }
                Msg::Ch1MeasuredValues(data) => {
                    format!("CH1 measured data: {}", data)
                }
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    };

    Cmp::new(config)
}
