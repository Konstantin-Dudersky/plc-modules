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
    pub dia_0: bool,
    pub dia_1: bool,
    pub dia_2: bool,
    pub dia_3: bool,
    pub dia_4: bool,
    pub dia_5: bool,
    pub dia_6: bool,
    pub dia_7: bool,
    pub dib_0: bool,
    pub dib_1: bool,
    pub dib_2: bool,
    pub dib_3: bool,
    pub dib_4: bool,
    pub dib_5: bool,
    pub dib_6: bool,
    pub dib_7: bool,
}

impl BufferBound for Buffer {}
