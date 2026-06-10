use serde::{Deserialize, Serialize};

/// Выходные данные
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OutputData {
    /// Канал CH0
    pub ch00: bool,

    /// Канал CH1
    pub ch01: bool,

    /// Канал CH2
    pub ch02: bool,

    /// Канал CH3
    pub ch03: bool,

    /// Канал CH4
    pub ch04: bool,

    /// Канал CH5
    pub ch05: bool,

    /// Канал CH6
    pub ch06: bool,

    /// Канал CH7
    pub ch07: bool,

    /// Канал CH8
    pub ch08: bool,

    /// Канал CH9
    pub ch09: bool,

    /// Канал CH10
    pub ch10: bool,

    /// Канал CH11
    pub ch11: bool,

    /// Канал CH12
    pub ch12: bool,

    /// Канал CH13
    pub ch13: bool,

    /// Канал CH14
    pub ch14: bool,

    /// Канал CH15
    pub ch15: bool,
}
