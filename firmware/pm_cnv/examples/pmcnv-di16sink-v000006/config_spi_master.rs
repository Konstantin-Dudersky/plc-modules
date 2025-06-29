use std::time::Duration;

use rsiot::{components::cmp_linux_spi_master::*, executor::Component};

use pm_cnv::pmcnv_di16sink_v000006;

use super::messages::*;

pub fn cmp() -> Component<Config<MRoot>, MRoot> {
    let di16_comm = ConfigDevicesCommSettings {
        linux_device: LinuxDevice::SpiWithCs {
            dev_spi: "/dev/spidev0.0".into(),
            dev_gpio: "/dev/gpiochip0".into(),
            gpio_line: 5,
        },
        baudrate: 1_000_000,
        spi_mode: ConfigDeviceSpiMode::Mode0,
    };

    let di16 = pmcnv_di16sink_v000006::Device {
        update_period: Duration::from_millis(100),
        fn_output: |buffer| {
            vec![
                MRoot::InputDia0(buffer.read.input_states.dia_0),
                MRoot::InputDia1(buffer.read.input_states.dia_1),
                MRoot::InputDia2(buffer.read.input_states.dia_2),
                MRoot::InputDia3(buffer.read.input_states.dia_3),
                MRoot::InputDia4(buffer.read.input_states.dia_4),
                MRoot::InputDia5(buffer.read.input_states.dia_5),
                MRoot::InputDia6(buffer.read.input_states.dia_6),
                MRoot::InputDia7(buffer.read.input_states.dia_7),
                MRoot::InputDib0(buffer.read.input_states.dib_0),
                MRoot::InputDib1(buffer.read.input_states.dib_1),
                MRoot::InputDib2(buffer.read.input_states.dib_2),
                MRoot::InputDib3(buffer.read.input_states.dib_3),
                MRoot::InputDib4(buffer.read.input_states.dib_4),
                MRoot::InputDib5(buffer.read.input_states.dib_5),
                MRoot::InputDib6(buffer.read.input_states.dib_6),
                MRoot::InputDib7(buffer.read.input_states.dib_7),
            ]
            .into_iter()
            .collect()
        },
    };

    let config = Config {
        devices_comm_settings: vec![di16_comm],
        devices: vec![Box::new(di16)],
    };

    Cmp::new(config)
}
