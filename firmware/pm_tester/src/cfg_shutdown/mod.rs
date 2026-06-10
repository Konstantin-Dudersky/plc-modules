use rsiot::components::cmp_shutdown::*;

use crate::msg::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        fn_input: |msg| {
            if let Msg::MsgDerive(MsgDerive::NeedShutdown(v)) = msg {
                v
            } else {
                false
            }
        },
    };

    Cmp::new(config)
}
