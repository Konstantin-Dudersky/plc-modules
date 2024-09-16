mod config_esp_i2c_slave;
mod config_esp_spi_master;
mod main;
mod message;

pub use main::main;

use message::Custom;
use pm_firmware_lib::pm_rq8_v0_0_3::{I2cRequest, I2cResponse};
