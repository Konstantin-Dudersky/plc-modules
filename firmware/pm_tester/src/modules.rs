use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum Module {
    #[default]
    None,
    PMCNV_DIx16_v000010,
}
