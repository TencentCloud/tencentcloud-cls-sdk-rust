#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("transport error")]
    Transport(#[from] reqwest::Error),
    #[error("bad url")]
    BadUrl(#[from] url::ParseError),
    #[error("missing header")]
    MissingHeader(Option<reqwest::header::HeaderName>),
    #[error("HTTP status {status_code}, code: {error_code}, message: {error_message}")]
    Endpoint {
        status_code: reqwest::StatusCode,
        error_code: String,
        error_message: String,
    },
    #[error("protobuf error")]
    Protobuf(#[from] quick_protobuf::Error),
}
