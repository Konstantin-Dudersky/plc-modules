use rsiot::{components::cmp_derive::*, executor::Component, message::Message};

use super::message::*;

const COEF: f64 = 24615.0;

pub fn new() -> Component<Config<MRoot>, MRoot> {
    let config = Config::<MRoot> {
        derive_items: vec![Box::new(DeriveItem::<MRoot, Buffer> {
            store: Buffer::default(),
            fn_input: |msg, buffer| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                if let MRoot::SpiMaster(msg) = msg {
                    match msg {
                        MSpiMaster::ValueCh0(v) => buffer.ch0 = v,
                        MSpiMaster::ValueCh1(v) => buffer.ch1 = v,
                        MSpiMaster::ValueCh2(v) => buffer.ch2 = v,
                        MSpiMaster::ValueCh3(v) => buffer.ch3 = v,
                    }
                }
            },
            fn_output: |buffer| {
                let msg1 = Message::new_custom(MRoot::Derive(MDerive::RawValue {
                    ch0: buffer.ch0,
                    ch1: buffer.ch1,
                    ch2: buffer.ch2,
                    ch3: buffer.ch3,
                }));
                let msg2 = Message::new_custom(MRoot::Derive(MDerive::AllValues {
                    sum: (buffer.ch0 + buffer.ch1 + buffer.ch2 + buffer.ch3) * COEF,
                    ch0: buffer.ch0 * COEF,
                    ch1: buffer.ch1 * COEF,
                    ch2: buffer.ch2 * COEF,
                    ch3: buffer.ch3 * COEF,
                }));
                Some(vec![msg1, msg2])
            },
        })],
    };

    Cmp::new(config)
}

#[derive(Clone, Default, PartialEq)]
struct Buffer {
    pub ch0: f64,
    pub ch1: f64,
    pub ch2: f64,
    pub ch3: f64,
}
