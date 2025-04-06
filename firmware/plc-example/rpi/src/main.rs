mod config_livecounter;

use std::time::Duration;

use rsiot::{
    components::{cmp_inject_periodic, cmp_linux_uart_master, cmp_livecounter, cmp_logger},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::Message,
};
use tracing::Level;

use messages::*;
use pm_uart_integration::pm_rq8_v0_0_5;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let config_livecounter = config_livecounter::config();

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(data) = msg.get_custom_data() else {
                return Ok(None);
            };
            match data {
                Custom::CounterEsp(_) => Ok(Some(msg.serialize_data()?)),
                _ => Ok(None),
            }
        },
    };

    // cmp_inject_periodic -------------------------------------------------------------------------
    let mut counter: u32 = 0;
    let mut output_0: bool = false;
    let config_inject_periodic = cmp_inject_periodic::Config {
        period: Duration::from_millis(2000),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::CounterRpi(counter));
            counter += 1;

            let msg2 = Message::new_custom(Custom::Hmi(Hmi::SetRelay0(output_0)));
            output_0 = !output_0;
            vec![msg, msg2]
        },
    };

    // cmp_linux_uart ------------------------------------------------------------------------------
    let config_linux_uart = cmp_linux_uart_master::Config::<_> {
        port: "/dev/ttyAMA0",
        baudrate: cmp_linux_uart_master::Baudrate::_115_200,
        data_bits: cmp_linux_uart_master::DataBits::_8,
        stop_bits: cmp_linux_uart_master::StopBits::_1,
        parity: cmp_linux_uart_master::Parity::None,
        timeout: Duration::from_millis(20),
        gpio_chip: "/dev/gpiochip0",
        pin_rts: Some(23),
        devices: vec![Box::new(pm_rq8_v0_0_5::Device {
            address: 1,
            periodic_request: Duration::from_millis(500),
            fn_input: |msg, buffer| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                match msg {
                    Custom::Hmi(msg) => match msg {
                        Hmi::SetRelay0(output_0) => {
                            buffer.output_0 = output_0;
                        }
                        Hmi::SetRelay1(output_1) => buffer.output_1 = output_1,
                    },
                    Custom::MasterLiveCounter(counter) => buffer.master_live_counter = counter,
                    _ => {}
                }
            },
            fn_output: |_| vec![],
        })],
    };

    // executor ------------------------------------------------------------------------------------
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Services::Rpi,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom, Services>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_linux_uart_master::Cmp::new(config_linux_uart))
        .add_cmp(cmp_inject_periodic::Cmp::new(config_inject_periodic))
        .add_cmp(cmp_livecounter::Cmp::new(config_livecounter))
        .wait_result()
        .await
        .unwrap();
}
