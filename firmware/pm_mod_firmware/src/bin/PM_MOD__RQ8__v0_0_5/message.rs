use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Custom {
    SetOutputs(u8),
    SlaveLiveCounter(u8),
    MasterLiveCounter(u8),
    /// Состояние соединения. 1 = соединение установлено
    ConnectionState(bool),
}

impl MsgDataBound for Custom {}
