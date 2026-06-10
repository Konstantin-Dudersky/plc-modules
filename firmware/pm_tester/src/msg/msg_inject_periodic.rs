use rsiot::message::MsgKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgInjPeriodic {
    I2cDetect(()),
    Watchdog(bool),
}
