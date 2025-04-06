use rsiot::message::{MsgDataBound, MsgKey, ServiceBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    CounterEsp(u32),
    CounterRpi(u32),
    Hmi(Hmi),
    MasterLiveCounter(u8),
    ConnectionState(bool),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Hmi {
    SetRelay0(bool),
    SetRelay1(bool),
}

impl MsgDataBound for Custom {
    type TService = Services;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Services {
    Esp,
    Rpi,
}

impl ServiceBound for Services {}
