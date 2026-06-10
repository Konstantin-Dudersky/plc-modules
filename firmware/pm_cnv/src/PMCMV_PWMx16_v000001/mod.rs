mod buffer;
mod device;
mod request_kind;

pub use {
    buffer::{Buffer, Channel},
    device::Device,
};

const DEVICE_NAME: &str = "PMCMV_PWMx16";
