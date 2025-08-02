use rsiot::{components::cmp_linux_spi_master::*, executor::Component};

use pm_cnv::pmcnv_ai4w_v000002;

use super::messages::*;

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

    let ai4w = pmcnv_ai4w_v000002::Device {
        fn_input: |msg, buffer| (),
        fn_output: |buffer| vec![],
    };

    let config = Config {
        devices_comm_settings: vec![ai4w_comm],
        devices: vec![Box::new(ai4w)],
    };

    Cmp::new(config)
}
