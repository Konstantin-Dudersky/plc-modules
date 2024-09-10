use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cRequest {
    SetOutputs(u8),
}
