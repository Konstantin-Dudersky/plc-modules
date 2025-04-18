use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    Data {
        ch0: f64,
        ch1: f64,
        ch2: f64,
        ch3: f64,
    },
}
