use rsiot::message::MsgKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgFilesystem {
    Settings(crate::cfg_filesystem::Buffer),
}
