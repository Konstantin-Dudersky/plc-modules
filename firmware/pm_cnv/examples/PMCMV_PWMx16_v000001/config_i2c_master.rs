use std::time::Duration;

use rsiot::{
    components::cmp_linux_i2c_master::*, components_config::master_device::ConfigDeviceStateOutput,
};

use pm_cnv::PMCMV_PWMx16_v000001;

use super::messages::*;

pub fn cmp() -> Cmp<Msg> {
    let pwm16 = PMCMV_PWMx16_v000001::Device {
        address: I2cAddress::Direct { address: 0x44 },
        update_rate: 1000.0,
        fn_input: |msg, buffer| match msg {
            Msg::Ch15(value) => {
                buffer.write.ch14.pwm_duty_cycle = *value;
                buffer.write.ch15.pwm_duty_cycle = 100.0 - *value
            }
            Msg::PWMx16DeviceState(_) => {}
        },

        device_state_output: Some(ConfigDeviceStateOutput {
            fn_device_state: |device_state| Msg::PWMx16DeviceState(device_state),
            period: Duration::from_millis(1_000),
        }),
    };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![Box::new(pwm16)],
    };

    Cmp::new(config)
}
