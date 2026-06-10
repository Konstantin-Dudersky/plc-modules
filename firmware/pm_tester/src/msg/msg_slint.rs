use rsiot::message::MsgKey;
use serde::{Deserialize, Serialize};

use crate::modules::Module;

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgSlint {
    ChangeActiveModule(Module),
    SetWatchdogEnabled(bool),
    PmcnvDix16SetAddress(u8),
}
