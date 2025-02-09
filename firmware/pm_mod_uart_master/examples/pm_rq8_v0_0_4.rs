//! Пример работы с GPIO компьютера Raspberry Pi
//!
//! Скомпилировать и загрузить на целевую систему:
//!
//! ```bash
//! cargo build --example pm_rq8_v0_0_4 --target="aarch64-unknown-linux-gnu" --release; scp target/aarch64-unknown-linux-gnu/release/examples/pm_rq8_v0_0_4 user@target:/home/user/
//! ```
//!
//! Запустить на целевой системе:
//!
//! ```bash
//! ./pm_rq8_v0_0_4
//! ```

use std::time::Duration;

use rsiot::{
    components::{cmp_inject_periodic, cmp_linux_uart_master, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_service::Service, Message, MsgDataBound},
};
use serde::{Deserialize, Serialize};
use tracing::Level;

use pm_firmware_linux_uart_master::PM_RQ8_v0_0_4;
use pm_firmware_uart_shared::settings::MESSAGE_LEN;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        RelayState(bool),
    }

    impl MsgDataBound for Custom {
        type TService = Service;
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    // cmp_linux_uart ------------------------------------------------------------------------------
    let config_linux_uart = cmp_linux_uart_master::Config::<_, MESSAGE_LEN> {
        port: "/dev/ttyUSB0",
        baudrate: cmp_linux_uart_master::Baudrate::_115_200,
        data_bits: cmp_linux_uart_master::DataBits::_8,
        stop_bits: cmp_linux_uart_master::StopBits::_2,
        parity: cmp_linux_uart_master::Parity::None,
        wait_after_write: Duration::from_millis(50),
        gpio_chip: "/dev/gpiochip0",
        pin_rts: None,
        devices: vec![Box::new(PM_RQ8_v0_0_4 {
            address: 20,
            fn_input: |msg, buffer| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                match msg {
                    Custom::RelayState(relay_state) => buffer.output_0 = relay_state,
                }
            },
            fn_output: |_| vec![],
        })],
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut relay_state = false;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(2000),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::RelayState(relay_state));
            relay_state = !relay_state;
            vec![msg]
        },
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_linux_uart_master::Cmp::new(config_linux_uart))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .wait_result()
        .await
        .unwrap();
}
