use rsiot::components_config::uart_general::RequestResponseBound;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UartRequest {
    SetOutputs(u8),
}

impl RequestResponseBound for UartRequest {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UartResponse {
    Ok,
}

impl RequestResponseBound for UartResponse {}
