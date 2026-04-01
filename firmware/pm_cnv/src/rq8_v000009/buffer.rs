use bitvec::{order::Msb0, view::BitView};
use rsiot::components_config::master_device::BufferBound;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub address: u8,
    pub write: Write,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Write {
    pub rq0: bool,
    pub rq1: bool,
    pub rq2: bool,
    pub rq3: bool,
    pub rq4: bool,
    pub rq5: bool,
    pub rq6: bool,
    pub rq7: bool,
}
impl Write {
    pub fn reg_a(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Msb0>();
        byte_view.set(0, self.rq0);
        byte_view.set(1, self.rq1);
        byte_view.set(2, self.rq2);
        byte_view.set(3, self.rq3);
        byte_view.set(4, self.rq4);
        byte_view.set(5, self.rq5);
        byte_view.set(6, self.rq6);
        byte_view.set(7, self.rq7);
        byte
    }
}

impl BufferBound for Buffer {}
