use rsiot::{components::cmp_linux_i2c_master::*, executor::Component};

use pm_cnv::rq8_v000006::Device;

use super::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let dq16 = Device {
        address: 0x22,
        fn_input: |msg, buffer| match msg {
            Msg::OutputRq0(v) => buffer.write.rq0 = *v,
            Msg::OutputRq1(v) => buffer.write.rq1 = *v,
            Msg::OutputRq2(v) => buffer.write.rq2 = *v,
            Msg::OutputRq3(v) => buffer.write.rq3 = *v,
            Msg::OutputRq4(v) => buffer.write.rq4 = *v,
            Msg::OutputRq5(v) => buffer.write.rq5 = *v,
            Msg::OutputRq6(v) => buffer.write.rq6 = *v,
            Msg::OutputRq7(v) => buffer.write.rq7 = *v,
        },
    };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![Box::new(dq16)],
    };

    Cmp::new(config)
}
