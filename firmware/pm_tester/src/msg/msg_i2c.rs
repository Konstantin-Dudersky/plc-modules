use rsiot::{components::shared_tasks::fieldbus_execution::FieldbusDiag, message::MsgKey};
use serde::{Deserialize, Serialize};

use pm_cnv::PMCNV_DIx16_v000010;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MsgI2c {
    Diag(FieldbusDiag),
    PMCNV_DIx16_data(PMCNV_DIx16_v000010::OutputData),
}
