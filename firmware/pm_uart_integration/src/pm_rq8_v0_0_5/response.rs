use super::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    SlaveLiveCounter(u8),
    Ok,
}
