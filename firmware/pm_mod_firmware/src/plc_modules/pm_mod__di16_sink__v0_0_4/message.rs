use rsiot::message::{Deserialize, MsgDataBound, Serialize};

use crate::Service;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    AllInputs([bool; 16]),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
