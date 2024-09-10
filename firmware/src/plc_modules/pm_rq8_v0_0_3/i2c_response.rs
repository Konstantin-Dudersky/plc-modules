use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cResponse {
    Ok,
}
