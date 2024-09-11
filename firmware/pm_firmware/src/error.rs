use esp_idf_svc::sys::EspError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    I2cSlave(EspError),

    #[error(transparent)]
    Postcard(#[from] super::postcard_serde::Error),
}
