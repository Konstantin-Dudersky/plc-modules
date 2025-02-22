use super::{BitView, BufferBound, Msb0};

/// Буфер данных коммуникации с модулем PM-RQ8
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Buffer {
    /// Состояние выхода 0
    pub output_0: bool,
    /// Состояние выхода 1
    pub output_1: bool,
    /// Состояние выхода 2
    pub output_2: bool,
    /// Состояние выхода 3
    pub output_3: bool,
    /// Состояние выхода 4
    pub output_4: bool,
    /// Состояние выхода 5
    pub output_5: bool,
    /// Состояние выхода 6
    pub output_6: bool,
    /// Состояние выхода 7
    pub output_7: bool,
    pub master_live_counter: u8,
    pub slave_live_counter: u8,
}

impl BufferBound for Buffer {}

impl Buffer {
    pub fn outputs_to_u8(&self) -> u8 {
        let mut output: u8 = 0;
        let bits = output.view_bits_mut::<Msb0>();
        bits.set(0, self.output_0);
        bits.set(1, self.output_1);
        bits.set(2, self.output_2);
        bits.set(3, self.output_3);
        bits.set(4, self.output_4);
        bits.set(5, self.output_5);
        bits.set(6, self.output_6);
        bits.set(7, self.output_7);
        output
    }
}
