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
    pub rq_1: bool,
    pub rq_2: bool,
    pub rq_3: bool,
    pub rq_4: bool,
    pub rq_5: bool,
    pub rq_6: bool,
    pub rq_7: bool,
}
impl Write {
    pub fn reg_a(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Msb0>();
        byte_view.set(0, self.dqb_0);
        byte_view.set(1, self.dqb_1);
        byte_view.set(2, self.dqb_2);
        byte_view.set(3, self.dqb_3);
        byte_view.set(4, self.dqb_4);
        byte_view.set(5, self.dqb_5);
        byte_view.set(6, self.dqb_6);
        byte_view.set(7, self.dqb_7);
        byte
    }

    pub fn reg_b(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Msb0>();
        byte_view.set(0, self.dqa_0);
        byte_view.set(1, self.dqa_1);
        byte_view.set(2, self.dqa_2);
        byte_view.set(3, self.dqa_3);
        byte_view.set(4, self.dqa_4);
        byte_view.set(5, self.dqa_5);
        byte_view.set(6, self.dqa_6);
        byte_view.set(7, self.dqa_7);
        byte
    }
}

impl BufferBound for Buffer {}
