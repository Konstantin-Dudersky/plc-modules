use rsiot::serde_utils::postcard_serde;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Postcard(#[from] postcard_serde::Error),
}
