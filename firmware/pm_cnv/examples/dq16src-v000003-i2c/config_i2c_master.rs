use rsiot::{components::cmp_linux_i2c_master::*, executor::Component};

use pm_cnv::dq16src_v000003::Device;

use super::messages::*;

pub fn cmp() -> Component<Config<Msg>, Msg> {
    let dq16 = Device {
        address: 0x20,
        fn_input: |msg, buffer| match msg {
            Msg::OutputDia0(v) => buffer.write.dqa_0 = *v,
            Msg::OutputDia1(v) => buffer.write.dqa_1 = *v,
            Msg::OutputDia2(v) => buffer.write.dqa_2 = *v,
            Msg::OutputDia3(v) => buffer.write.dqa_3 = *v,
            Msg::OutputDia4(v) => buffer.write.dqa_4 = *v,
            Msg::OutputDia5(v) => buffer.write.dqa_5 = *v,
            Msg::OutputDia6(v) => buffer.write.dqa_6 = *v,
            Msg::OutputDia7(v) => buffer.write.dqa_7 = *v,
            Msg::OutputDib0(v) => buffer.write.dqb_0 = *v,
            Msg::OutputDib1(v) => buffer.write.dqb_1 = *v,
            Msg::OutputDib2(v) => buffer.write.dqb_2 = *v,
            Msg::OutputDib3(v) => buffer.write.dqb_3 = *v,
            Msg::OutputDib4(v) => buffer.write.dqb_4 = *v,
            Msg::OutputDib5(v) => buffer.write.dqb_5 = *v,
            Msg::OutputDib6(v) => buffer.write.dqb_6 = *v,
            Msg::OutputDib7(v) => buffer.write.dqb_7 = *v,
        },
    };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![Box::new(dq16)],
    };

    Cmp::new(config)
}
