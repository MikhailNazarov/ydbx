mod generated;

pub use generated::*;

use tonic::metadata::{Ascii, MetadataValue};
use tonic::service::Interceptor;

/// Publicly re-export some types from tonic which users might need
/// for implementing traits, or for naming concrete client types.
pub mod tonic_exports {
    pub use tonic::service::interceptor::InterceptedService;
    pub use tonic::transport::Channel;
    pub use tonic::transport::Endpoint;
    pub use tonic::Status;
}

/// Helper trait for types or closures that can provide authentication
/// tokens for YDB
pub trait TokenProvider {
    /// Fetch a currently valid authentication token for Yandex Cloud.
    fn get_token<'a>(&'a mut self) -> Result<&'a str, tonic::Status>;
}

impl TokenProvider for String {
    fn get_token<'a>(&'a mut self) -> Result<&'a str, tonic::Status> {
        Ok(self.as_str())
    }
}

impl TokenProvider for &'static str {
    fn get_token(&mut self) -> Result<&'static str, tonic::Status> {
        Ok(*self)
    }
}

/// Interceptor for adding authentication headers to gRPC requests.
/// This is constructed with a callable that returns authentication
/// tokens.
///
/// This callable is responsible for ensuring that the returned tokens
/// are valid at the given time, i.e. it should take care of
/// refreshing and so on.
pub struct AuthInterceptor<T: TokenProvider> {
    token_provider: T,
}

impl<T: TokenProvider> AuthInterceptor<T> {
    pub fn new(token_provider: T) -> Self {
        Self { token_provider }
    }
}

impl<T: TokenProvider> Interceptor for AuthInterceptor<T> {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        let token: MetadataValue<Ascii> = format!("Bearer {}", self.token_provider.get_token()?)
            .try_into()
            .map_err(|_| {
                tonic::Status::invalid_argument("authorization token contained invalid characters")
            })?;

        request.metadata_mut().insert("authorization", token);

        Ok(request)
    }
}
