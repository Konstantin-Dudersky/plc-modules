mod config_derive;
mod config_esp_gpio;
mod config_esp_i2c_slave;
mod config_esp_spi_master;
mod config_logger;
mod main;
mod message;

pub use main::main;
use message::Custom;
use pm_firmware_lib::pm_di16_dc24sink_v0_0_2::{I2cRequest, I2cResponse};
