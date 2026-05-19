use bitvec::{order::Lsb0, view::BitView};
use rsiot::components_config::{i2c_master::I2cAddress, master_device::BufferBound};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub address: I2cAddress,
    pub write: Write,
    pub read: Read,

    // Строка кнопок для чтения
    pub read_row: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Write {
    pub led0: bool,
    pub led1: bool,
    pub led2: bool,
    pub led3: bool,
    pub led4: bool,
    pub led5: bool,
    pub led6: bool,
    pub led7: bool,
    pub button_row0: bool,
    pub button_row1: bool,
}
impl Write {
    pub fn reg_a(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Lsb0>();
        byte_view.set(0, self.button_row0);
        byte_view.set(1, false);
        byte_view.set(2, false);
        byte_view.set(3, false);
        byte_view.set(4, self.led0);
        byte_view.set(5, self.led1);
        byte_view.set(6, self.led2);
        byte_view.set(7, self.led3);
        byte
    }
    pub fn reg_b(&self) -> u8 {
        let mut byte = 0;
        let byte_view = byte.view_bits_mut::<Lsb0>();
        byte_view.set(0, self.led4);
        byte_view.set(1, self.led5);
        byte_view.set(2, self.led6);
        byte_view.set(3, self.led7);
        byte_view.set(4, false);
        byte_view.set(5, false);
        byte_view.set(6, false);
        byte_view.set(7, self.button_row1);
        byte
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Read {
    pub pressed_button: PressedButton,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PressedButton {
    #[default]
    None,
    Row0Col0,
    Row0Col1,
    Row0Col2,
    Row0Col3,
    Row0Col4,
    Row0Col5,
    Row1Col0,
    Row1Col1,
    Row1Col2,
    Row1Col3,
    Row1Col4,
    Row1Col5,
}

impl BufferBound for Buffer {}
