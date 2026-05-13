use std::time::Duration;

use rsiot::{
    components::cmp_linux_i2c_master::*, components_config::master_device::ConfigDeviceStateOutput,
};

use pm_cnv::PMCMV_INA226x4_v000001;

use super::messages::*;

pub fn cmp() -> Cmp<Msg> {
    let ch0 = PMCMV_INA226x4_v000001::Device {
        address: 0x40,
        shunt_resistance: 0.050,
        max_current: 5.0,
        average_mode: PMCMV_INA226x4_v000001::AveragingMode::_1,
        bus_voltage_conversion_time: PMCMV_INA226x4_v000001::ConversionTime::_1100us,
        shunt_voltage_conversion_time: PMCMV_INA226x4_v000001::ConversionTime::_1100us,
        fn_output: |buffer| {
            let msg = Msg::Ch0MeasuredValues(buffer.output_data());
            vec![msg]
        },
        device_state_output: ConfigDeviceStateOutput {
            fn_device_state: Msg::Ch0DeviceState,
            period: Duration::from_millis(1_000),
        },
    };

    // let ch1 = PMCMV_INA226x4_v000001::Device {
    //     address: 0x41,
    //     shunt_resistance: 0.050,
    //     max_current: 5.0,
    //     average_mode: PMCMV_INA226x4_v000001::AveragingMode::_1024,
    //     bus_voltage_conversion_time: PMCMV_INA226x4_v000001::ConversionTime::_4156us,
    //     shunt_voltage_conversion_time: PMCMV_INA226x4_v000001::ConversionTime::_4156us,
    //     fn_output: |buffer| {
    //         let msg = Msg::Ch1MeasuredValues(buffer.output_data());
    //         vec![msg]
    //     },
    //     device_state_output: ConfigDeviceStateOutput {
    //         fn_device_state: Msg::Ch1DeviceState,
    //         period: Duration::from_millis(1_000),
    //     },
    // };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![
            Box::new(ch0),
            //    Box::new(ch1)
        ],
    };

    Cmp::new(config)
}
