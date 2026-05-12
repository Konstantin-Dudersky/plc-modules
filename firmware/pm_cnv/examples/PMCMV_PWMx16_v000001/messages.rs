use rsiot::{
    components_config::master_device::DeviceState,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Ch15(f32),
    PWMx16DeviceState(DeviceState),
}

impl MsgDataBound for Msg {}
