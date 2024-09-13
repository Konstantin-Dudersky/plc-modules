mod define_address;
mod error;
mod i2c_slave;
pub mod plc_modules;
mod postcard_serde;
mod service;
mod spi_devices;

pub use define_address::define_address;
use service::Service;
