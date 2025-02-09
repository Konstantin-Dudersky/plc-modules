//! Устройства для применения в UART-мастере

mod pm_rq8_v0_0_4;
pub use pm_rq8_v0_0_4::PM_RQ8_v0_0_4;

type Result<T> = std::result::Result<T, rsiot::components_config::uart_master::Error>;
