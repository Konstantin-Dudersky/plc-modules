use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    AllInputs([bool; 16]),
}

impl MsgDataBound for Custom {}
