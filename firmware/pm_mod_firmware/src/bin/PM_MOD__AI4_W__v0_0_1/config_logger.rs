use rsiot::components::cmp_logger;

use super::message::*;

pub fn config() -> cmp_logger::Config<MRoot> {
    cmp_logger::Config {
        level: cmp_logger::Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };

            let text = match msg {
                MRoot::Derive(MDerive::RawValue { ch0, ch1, ch2, ch3 }) => {
                    format!("CH0: {ch0:>9.6}, CH1: {ch1:>9.6}, CH2: {ch2:>9.6}, CH3: {ch3:>9.6}")
                }
                _ => return Ok(None),
            };

            Ok(Some(text))
        },
    }
}
