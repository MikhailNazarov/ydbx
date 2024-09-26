use thiserror::Error;
use tracing::error;
use ydb::{YdbError, YdbOrCustomerError};

#[derive(Error, Debug)]
pub enum YdbxError {
    #[error(transparent)]
    YdbError(#[from] YdbError),
    #[error(transparent)]
    YdbOrCustomerError(#[from] YdbOrCustomerError),

    #[error(transparent)]
    TimeoutError(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    IntParseError(#[from] std::num::ParseIntError),
}
