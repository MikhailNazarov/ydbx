use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum YdbxError {
    #[error(transparent)]
    TimeoutError(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    IntParseError(#[from] std::num::ParseIntError),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Session error: {0}")]
    SessionError(String),
}
