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
    pub pressed_button: Option<(u8, u8)>,
    pub pressed_touch: Option<(u32, u32)>,
}
