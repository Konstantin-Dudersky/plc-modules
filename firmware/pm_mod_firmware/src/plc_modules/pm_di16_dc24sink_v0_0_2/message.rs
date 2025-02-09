use rsiot::message::{Deserialize, MsgDataBound, Serialize};

use crate::Service;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    GetInput,
    InputsState { a: u8, b: u8 },
    LedState { a: u8, b: u8 },
    GpioExpanderInt(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
