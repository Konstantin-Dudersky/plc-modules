use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Response {
    Data {
        pressed_button: Option<(u8, u8)>,
        pressed_touch: Option<(u32, u32)>,
    },
}
