use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cRequest {
    GetInput,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cResponse {
    InputsState {
        a0: bool,
        a1: bool,
        a2: bool,
        a3: bool,
        a4: bool,
        a5: bool,
        a6: bool,
        a7: bool,
        b0: bool,
        b1: bool,
        b2: bool,
        b3: bool,
        b4: bool,
        b5: bool,
        b6: bool,
        b7: bool,
    },
}
