use rsiot::{
    components::shared_tasks::fieldbus_execution::FieldbusDiag,
    message::{MsgDataBound, MsgKey},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
pub enum Msg {
    OutputDia0(bool),
    OutputDia1(bool),
    OutputDia2(bool),
    OutputDia3(bool),
    OutputDia4(bool),
    OutputDia5(bool),
    OutputDia6(bool),
    OutputDia7(bool),
    OutputDib0(bool),
    OutputDib1(bool),
    OutputDib2(bool),
    OutputDib3(bool),
    OutputDib4(bool),
    OutputDib5(bool),
    OutputDib6(bool),
    OutputDib7(bool),
    Diag(FieldbusDiag),
}

impl MsgDataBound for Msg {}
