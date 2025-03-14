use rsiot::message::{Deserialize, MsgDataBound, Serialize};

use crate::Service;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    SetOutputs1(u8),
    SetOutputs2(u8),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
