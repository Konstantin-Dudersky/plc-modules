use rsiot::message::{Deserialize, MsgDataBound, Serialize};

use crate::Service;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    SetOutputs1([bool; 8]),
    SetOutputs2([bool; 8]),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
