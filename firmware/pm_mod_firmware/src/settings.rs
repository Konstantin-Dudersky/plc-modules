use std::time::Duration;

use rsiot::components_config::uart_general;

/// Компиляция
pub const DEBUG: bool = false;

pub const UART_BAUDRATE: uart_general::Baudrate = uart_general::Baudrate::_115_200;
pub const UART_DATABITS: uart_general::DataBits = uart_general::DataBits::_8;
pub const UART_STOPBITS: uart_general::StopBits = uart_general::StopBits::_1;
pub const UART_PARITY: uart_general::Parity = uart_general::Parity::None;

pub const SPI_BAUDRATE: u32 = 1_000_000;

pub const LIVECOUNTER_GENERATE: Duration = Duration::from_millis(500);
pub const LIVECOUNTER_CHECK: Duration = Duration::from_millis(2000);
