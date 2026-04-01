use rsiot::components_config::master_device::BufferBound;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub address: u8,
    pub read: Read,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub input_states: InputStates,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputStates {
    pub ch00: bool,
    pub ch01: bool,
    pub ch02: bool,
    pub ch03: bool,
    pub ch04: bool,
    pub ch05: bool,
    pub ch06: bool,
    pub ch07: bool,
    pub ch08: bool,
    pub ch09: bool,
    pub ch10: bool,
    pub ch11: bool,
    pub ch12: bool,
    pub ch13: bool,
    pub ch14: bool,
    pub ch15: bool,
}

impl BufferBound for Buffer {}
