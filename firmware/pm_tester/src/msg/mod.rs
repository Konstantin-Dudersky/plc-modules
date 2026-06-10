mod msg_derive;
mod msg_filesystem;
mod msg_i2c;
mod msg_inject_periodic;
mod msg_inject_single;
mod msg_os_process;
mod msg_slint;

pub use {
    msg_derive::MsgDerive, msg_filesystem::MsgFilesystem, msg_i2c::MsgI2c,
    msg_inject_periodic::MsgInjPeriodic, msg_inject_single::MsgInjectSingle,
    msg_os_process::MsgOsProcess, msg_slint::MsgSlint,
};

use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    MsgDerive(MsgDerive),
    MsgI2c(MsgI2c),
    MsgInjPeriodic(MsgInjPeriodic),
    MsgInjectSingle(MsgInjectSingle),
    MsgSlint(MsgSlint),
    MsgOsProcess(MsgOsProcess),
    MsgFilesystem(MsgFilesystem),
}

impl MsgDataBound for Msg {}
