use rsiot::message::MsgKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgDerive {
    Watchdog {
        watchdog_enabled: bool,
        watchdog_signal: bool,
    },

    NeedShutdown(bool),
}
