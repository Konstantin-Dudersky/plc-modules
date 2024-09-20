use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cRequest {
    SetOutputs(u8),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cResponse {
    Ok(u32),
}
