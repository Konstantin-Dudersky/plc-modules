use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    OutputRq0(bool),
    OutputRq1(bool),
    OutputRq2(bool),
    OutputRq3(bool),
    OutputRq4(bool),
    OutputRq5(bool),
    OutputRq6(bool),
    OutputRq7(bool),
}

impl MsgDataBound for Msg {}
