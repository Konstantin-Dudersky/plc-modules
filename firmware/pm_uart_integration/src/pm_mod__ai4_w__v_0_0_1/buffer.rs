use rsiot::components_config::{master_device::BufferBound, uart_general::protocol::Protocol};

/// Буфер данных коммуникации с модулем PM-RQ8
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Buffer {
    pub protocol: Protocol,
    pub read: Read,
}
impl BufferBound for Buffer {}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Read {
    pub value_ch0: f64,
    pub value_ch1: f64,
    pub value_ch2: f64,
    pub value_ch3: f64,
}
