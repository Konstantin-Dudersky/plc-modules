use rsiot::components::cmp_logger;

use super::Custom;

pub fn config() -> cmp_logger::Config<Custom> {
    cmp_logger::Config {
        level: cmp_logger::Level::INFO,
        fn_input: |msg| {
            let text = msg.data;
            let text = format!("Logger: {text:?}");
            Ok(Some(text))
        },
    }
}
