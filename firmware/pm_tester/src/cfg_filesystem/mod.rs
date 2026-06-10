use rsiot::{
    components::cmp_filesystem::{BufferBound, CallFnOutputKind, Cmp, Config},
    serde_utils::SerdeAlgKind,
};
use serde::{Deserialize, Serialize};

use crate::{CONFIG_FILE, msg::*};

pub fn cmp() -> Cmp<Msg, Buffer> {
    let config_filesystem = Config::<Msg, Buffer> {
        filename: CONFIG_FILE.into(),
        serde_alg: SerdeAlgKind::Toml,
        call_fn_output_kind: CallFnOutputKind::Always,
        fn_input: |msg, buffer| match msg {
            Msg::MsgSlint(msg) => match msg {
                MsgSlint::ChangeActiveModule(m) => {
                    let mut buffer = buffer.clone();
                    buffer.module = *m;
                    Some(buffer)
                }

                MsgSlint::PmcnvDix16SetAddress(a) => {
                    let mut buffer = buffer.clone();
                    buffer.pmcnv_dix16_v000010_address = *a;
                    Some(buffer)
                }
                _ => None,
            },
            _ => None,
        },
        fn_output: |buffer| vec![Msg::MsgFilesystem(MsgFilesystem::Settings(buffer.clone()))],
    };

    Cmp::new(config_filesystem)
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Buffer {
    pub module: crate::modules::Module,
    pub pmcnv_dix16_v000010_address: u8,
}

impl BufferBound for Buffer {}
