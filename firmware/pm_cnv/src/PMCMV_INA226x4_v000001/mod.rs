mod buffer;
mod device;
mod output_data;
mod request_kind;

pub use {
    crate::chips::ina226::{AveragingMode, ConversionTime},
    buffer::Buffer,
    device::Device,
    output_data::OutputData,
};

const DEVICE_NAME: &str = "PMCMV_PWMx16";
