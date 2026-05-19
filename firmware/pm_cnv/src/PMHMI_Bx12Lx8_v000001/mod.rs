mod buffer;
mod device;
mod request_kind;

pub use {
    buffer::{Buffer, PressedButton},
    device::Device,
};

const DEVICE_NAME: &str = "Bx12Lx8";
