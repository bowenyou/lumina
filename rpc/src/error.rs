pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(not(target_arch = "wasm32"))]
    #[error("Token contains invalid characters: {0}")]
    InvalidCharactersInToken(#[from] http::header::InvalidHeaderValue),

    #[error("Protocol not supported or missing: {0}")]
    ProtocolNotSupported(String),

    #[error(transparent)]
    JsonRpc(#[from] jsonrpsee::core::Error),
}
