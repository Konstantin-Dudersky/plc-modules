use strum::FromRepr;

#[derive(FromRepr)]
pub enum RequestKind {
    RequestAllRegisters,
    WriteConfigurationRegister,
    WriteModeRegister,
    WriteGPOCONRegister,
    ReadConfigurationRegister,
    ReadStatusRegister,
    ReadDataRegister,
    Reset,
    ReadIdRegister,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}

impl From<u8> for RequestKind {
    fn from(value: u8) -> Self {
        RequestKind::from_repr(value as usize).unwrap()
    }
}
