use rsiot::message::{Deserialize, MsgDataBound, Serialize};

use pm_mod_firmware::Service;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Custom {
    SetOutputs(u8),
    SlaveLiveCounter(u8),
    MasterLiveCounter(u8),
    /// Состояние соединения. 1 = соединение установлено
    ConnectionState(bool),
}

impl MsgDataBound for Custom {
    type TService = Service;
}
