use std::time::Duration;

use rsiot::components::cmp_linux_i2c_master::*;

use pm_cnv::PMCMV_PWMx16_v000001;

use super::msg::*;

pub fn cmp() -> Cmp<Msg> {
    let pwm16 = PMCMV_PWMx16_v000001::Device {
        address: I2cAddress::Direct { address: 0x44 },
        update_rate: 100.0,
        fn_input: |msg, buffer| match msg {
            Msg::Ch15(value) => {
                buffer.write.ch00.pwm_duty_cycle = 10.0;
                buffer.write.ch01.pwm_duty_cycle = 100.0 - *value
            }
            Msg::Diag(_) => {}
        },
    };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![Box::new(pwm16)],
        fn_diag: |diag| Msg::Diag(diag.clone()),
        fn_diag_period: Duration::from_millis(1_000),
    };

    Cmp::new(config)
}
