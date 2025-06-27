use rsiot::message::{MsgDataBound, MsgKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum MRoot {
    InputDia0(bool),
    InputDia1(bool),
    InputDia2(bool),
    InputDia3(bool),
    InputDia4(bool),
    InputDia5(bool),
    InputDia6(bool),
    InputDia7(bool),
    InputDib0(bool),
    InputDib1(bool),
    InputDib2(bool),
    InputDib3(bool),
    InputDib4(bool),
    InputDib5(bool),
    InputDib6(bool),
    InputDib7(bool),
}

impl MsgDataBound for MRoot {}
