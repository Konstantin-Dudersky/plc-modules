use rsiot::{
    components_config::master_device::DeviceState,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Ch0DeviceState(DeviceState),
    Ch1DeviceState(DeviceState),
    Ch0MeasuredValues(pm_cnv::PMCMV_INA226x4_v000001::OutputData),
    Ch1MeasuredValues(pm_cnv::PMCMV_INA226x4_v000001::OutputData),
}

impl MsgDataBound for Msg {}
