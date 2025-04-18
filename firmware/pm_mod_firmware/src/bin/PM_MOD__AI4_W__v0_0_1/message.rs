use rsiot::message::{Deserialize, MsgDataBound, MsgKey, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MRoot {
    Derive(MDerive),
    SpiMaster(MSpiMaster),
}

impl MsgDataBound for MRoot {}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MSpiMaster {
    ValueCh0(f64),
    ValueCh1(f64),
    ValueCh2(f64),
    ValueCh3(f64),
}

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MDerive {
    RawValue {
        ch0: f64,
        ch1: f64,
        ch2: f64,
        ch3: f64,
    },
    AllValues {
        sum: f64,
        ch0: f64,
        ch1: f64,
        ch2: f64,
        ch3: f64,
    },
}
