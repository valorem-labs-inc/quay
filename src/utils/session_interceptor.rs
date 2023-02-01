use tonic::metadata::AsciiMetadataValue;
use tonic::service::Interceptor;
use tonic::{Request, Status};

const COOKIE_HEADER_KEY: &str = "cookie";

/// The Session Interceptor is a gRPC interceptor for the client to add session
/// authentication details into the `request` header information such that the server can
/// validate/confirm the client is using a valid session.
#[derive(Default)]
pub struct SessionInterceptor {
    pub session_cookie: String,
}

impl Interceptor for SessionInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // If we have established a session set the appropriate session headers before the request
        // goes to the server.
        let request = if !self.session_cookie.is_empty() {
            let mut request = request;

            // We should always be able to transform a String into an `AsciiMetadataValue` so its
            // safe to unwrap without checking.
            let cookie_value = AsciiMetadataValue::try_from(&self.session_cookie).unwrap();

            // Insert the session cookie.
            request
                .metadata_mut()
                .insert(COOKIE_HEADER_KEY, cookie_value);
            request
        } else {
            request
        };

        Ok(request)
    }
}
