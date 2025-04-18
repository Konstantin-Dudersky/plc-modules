use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    SetOutputs1(u8),
    SetOutputs2(u8),
}

impl MsgDataBound for Custom {}
