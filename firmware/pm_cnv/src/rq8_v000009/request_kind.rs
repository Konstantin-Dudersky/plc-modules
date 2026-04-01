#[repr(u8)]
pub enum RequestKind {
    Init,
    SetOutputs,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}
