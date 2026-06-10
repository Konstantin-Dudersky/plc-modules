use rsiot::{
    components::shared_tasks::fieldbus_execution::FieldbusDiag,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    Diag(FieldbusDiag),
    Ch0MeasuredValues(pm_cnv::PMCMV_INA226x4_v000001::OutputData),
    Ch1MeasuredValues(pm_cnv::PMCMV_INA226x4_v000001::OutputData),
}

impl MsgDataBound for Msg {}
