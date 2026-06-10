//! Библиотека устройств для работы с модулями PMCNV, PMHMI, PMIFC

#![allow(non_snake_case)]
#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]

use rsiot::components_config::i2c_master::I2cAddress;

pub mod PMCMV_INA226x4_v000001;
pub mod PMCMV_PWMx16_v000001;
pub mod PMCNV_AIWx4_v000002;
pub mod PMCNV_DIx16_v000010;
pub mod PMCNV_RQx8_v000009;
pub mod PMHMI_Bx12Lx8_v000001;
pub mod PMIFC_I2Cx8_v000002;
pub mod chips;
pub mod dq16src_v000003;
pub mod pm_cnv__ai4_w__v0_0_1;
pub mod pm_cnv__keyboard__v0_0_2;
pub mod pmcnv_dq16src_v000002;

fn device_id_i2c(name: impl AsRef<str>, address: I2cAddress) -> String {
    format!("{} ({})", name.as_ref(), address)
}
