use rsiot::{components::cmp_derive, message::Message};

use super::Custom;

pub fn config() -> cmp_derive::Config<Custom> {
    #[derive(Clone, Default, PartialEq)]
    struct Buffer {
        pub a: Option<u8>,
        pub b: Option<u8>,
    }

    let item = cmp_derive::DeriveItem {
        store: Buffer::default(),
        fn_input: |msg, buffer| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                Custom::InputsState { a, b } => {
                    buffer.a = Some(a);
                    buffer.b = Some(b);
                }
                _ => return,
            }
        },
        fn_output: |buffer| {
            let msg = Message::new_custom(Custom::LedState {
                a: buffer.a?,
                b: buffer.b?,
            });
            Some(vec![msg])
        },
    };

    cmp_derive::Config {
        derive_items: vec![Box::new(item)],
    }
}
