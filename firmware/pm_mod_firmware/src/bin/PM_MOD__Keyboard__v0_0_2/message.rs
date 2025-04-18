use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MRoot {
    SpiMaster(MSpiMaster),
}

impl MsgDataBound for MRoot {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MSpiMaster {
    PressedButton(Option<(u8, u8)>),
    PressedTouch(Option<(u32, u32)>),
}
