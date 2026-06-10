use std::time::Duration;

use rsiot::{components::cmp_linux_spi_master::*, executor::Component};

use pm_cnv::PMCNV_AIWx4_v000002;

use super::msg::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let ai4w_comm = ConfigDevicesCommSettings {
        linux_device: LinuxDevice::SpiWithCs {
            dev_spi: "/dev/spidev0.0".into(),
            dev_gpio: "/dev/gpiochip0".into(),
            gpio_line: 4,
        },
        baudrate: 100_000,
        spi_mode: ConfigDeviceSpiMode::Mode3,
    };

    let ai4w = PMCNV_AIWx4_v000002::Device {
        fn_input: |_msg, _buffer| (),
        fn_output: |_buffer| vec![],
    };

    let config = Config {
        devices_comm_settings: vec![ai4w_comm],
        devices: vec![Box::new(ai4w)],
        fn_diag: |diag| Msg::Diag(diag.clone()),
        fn_diag_period: Duration::from_millis(1_000),
    };

    Cmp::new(config)
}
