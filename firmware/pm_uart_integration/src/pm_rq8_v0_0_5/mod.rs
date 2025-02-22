mod buffer;
mod device;
mod request;
mod response;

use async_trait::async_trait;
use bitvec::{order::Msb0, view::BitView};
use rsiot::{
    components_config::{
        master_device::{BufferBound, ConfigPeriodicRequest, DeviceBase, DeviceTrait, Result},
        uart_general::{UartRequest, UartResponse},
    },
    message::{Message, MsgDataBound},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

pub use buffer::Buffer;
pub use device::Device;
pub use request::Request;
pub use response::Response;
