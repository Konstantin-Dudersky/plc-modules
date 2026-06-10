use std::time::Duration;

use pm_cnv::PMCNV_DIx16_v000010;
use rsiot::components::cmp_linux_i2c_master::*;

use crate::{modules::Module, msg::*};

pub fn cmp(settings: crate::cfg_filesystem::Buffer) -> Cmp<Msg> {
    let device = match settings.module {
        Module::None => panic!("No module selected"),
        Module::PMCNV_DIx16_v000010 => PMCNV_DIx16_v000010::Device {
            address: I2cAddress::Direct {
                address: settings.pmcnv_dix16_v000010_address,
            },
            update_period: Duration::from_millis(100),
            fn_output: |buffer| vec![Msg::MsgI2c(MsgI2c::PMCNV_DIx16_data(buffer.output_data()))],
        },
    };

    let config = Config {
        dev_i2c: "/dev/i2c-0".into(),
        devices: vec![Box::new(device)],
        fn_diag: |d| Msg::MsgI2c(MsgI2c::Diag(d.clone())),
        fn_diag_period: Duration::from_millis(1_000),
    };

    Cmp::new(config)
}
