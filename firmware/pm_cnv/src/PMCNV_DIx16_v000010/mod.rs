//! Модуль PMCNV_DIx16_v0.0.10

mod buffer;
mod device;
mod output_data;
mod request_kind;

pub use {buffer::Buffer, device::Device, output_data::OutputData};

const DEVICE_NAME: &str = "PMCNV_DIx16";
