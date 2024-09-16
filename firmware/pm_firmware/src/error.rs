use rsiot::drivers_i2c::postcard_serde;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Postcard(#[from] postcard_serde::Error),
}
