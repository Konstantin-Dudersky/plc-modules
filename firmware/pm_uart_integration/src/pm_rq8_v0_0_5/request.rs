use super::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Request {
    SetOutputs(u8),
    SetMasterLiveCounter(u8),
    GetSlaveLiveCounter,
}
