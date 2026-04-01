use strum::FromRepr;

#[derive(FromRepr)]
pub enum RequestKind {
    Init,
    ReadInputs,
}
impl From<RequestKind> for u8 {
    fn from(value: RequestKind) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for RequestKind {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let result = RequestKind::from_repr(value as usize);
        match result {
            Some(kind) => Ok(kind),
            None => Err(anyhow::Error::msg("Invalid RequestKind: {value}")),
        }
    }
}
