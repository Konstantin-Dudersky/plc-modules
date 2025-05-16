use std::time::Duration;

use rsiot::{components::cmp_linux_spi_master::*, executor::Component, message::Message};

use pm_cnv::pm_cnv__di16_sink__v0_0_6;

use super::messages::*;

pub fn cmp() -> Component<Config<MRoot>, MRoot> {
    let di16_comm = ConfigDevicesCommSettings {
        // spi_adapter_path: "/dev/spidev1.0",
        spi_adapter_path: "/dev/spidev0.0",
        baudrate: 100_000,
        spi_mode: ConfigDeviceSpiMode::Mode0,
    };

    let di16 = pm_cnv__di16_sink__v0_0_6::Device {
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
            .map(Message::new_custom)
            .collect()
        },
    };

    let config = Config {
        devices_comm_settings: vec![di16_comm],
        devices: vec![Box::new(di16)],
    };

    Cmp::new(config)
}
