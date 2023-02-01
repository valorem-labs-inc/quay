use crate::auth::{
    unix_timestamp, SignedMessage, EXPIRATION_TIME_KEY, NONCE_KEY, USER_ADDRESS_KEY,
};
use crate::session::{session_server::Session, Empty, NonceText, VerifyText};

use axum_sessions::SessionHandle;
use ethers::prelude::Address;
use siwe::VerificationOpts;
use tokio::{sync::RwLockReadGuard, task};
use tonic::{service::Interceptor, Request, Response, Status};

const SEVEN_DAYS_IN_SECONDS: u64 = 604800u64;

// Private authentication function for used by the service endpoint and the session interceptor.
fn authenticate(
    session: &RwLockReadGuard<axum_sessions::async_session::Session>,
) -> Result<(), Status> {
    // Confirm the nonce is valid.
    match session.get::<String>(NONCE_KEY) {
        Some(_) => (),
        // Invalid nonce
        None => return Err(Status::unauthenticated("Failed to get nonce")),
    }

    // Confirm the session is still valid.
    let now = match unix_timestamp() {
        Ok(now) => now,
        Err(_) => return Err(Status::internal("Failed to get unix timestamp.")),
    };

    match session.get::<u64>(EXPIRATION_TIME_KEY) {
        None => return Err(Status::unauthenticated("Failed to get session expiration")),
        Some(ts) => {
            if now > ts {
                return Err(Status::unauthenticated("Session expired"));
            }
        }
    }

    // Authenticated request
    Ok(())
}

/// The SessionAuthenticator is a gRPC interceptor for the server to check and validate session
/// authentication details in the `request`.
#[derive(Clone)]
pub struct SessionAuthenticator;

impl Interceptor for SessionAuthenticator {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        // Use an internal scope to ensure the borrow of the request is dropped (i.e.
        // RwLockReadGuard is dropped at the end of the scope), in order to move it for the return
        // value
        {
            let session_handle = request.extensions().get::<SessionHandle>().unwrap();
            let session = task::block_in_place(|| session_handle.blocking_read());
            authenticate(&session)
        }
        .map(|_| request)
    }
}

#[derive(Debug, Default)]
pub struct SessionService;

#[tonic::async_trait]
impl Session for SessionService {
    async fn nonce(&self, request: Request<Empty>) -> Result<Response<NonceText>, Status> {
        // Fetch a writeable session.
        let session_handle = request.extensions().get::<SessionHandle>().unwrap();
        let mut session = session_handle.write().await;

        // Generate and set the nonce
        let nonce = siwe::generate_nonce();
        match session.insert(NONCE_KEY, &nonce) {
            Ok(_) => (),
            Err(_) => return Err(Status::internal("Failed to set nonce.")),
        }

        // Make sure we don't inherit a dirty session expiry
        let ts = match unix_timestamp() {
            Ok(ts) => ts,
            Err(_) => return Err(Status::internal("Failed to get unix timestamp.")),
        };

        // Set the expiry time in the session
        match session.insert(EXPIRATION_TIME_KEY, ts) {
            Ok(_) => (),
            Err(_) => return Err(Status::internal("Failed to set expiration.")),
        }

        // Send the response
        Ok(Response::new(NonceText { nonce }))
    }

    async fn verify(&self, request: Request<VerifyText>) -> Result<Response<Empty>, Status> {
        // Decode the JSON message body into the expected SignedMessage structure
        let signed_message: SignedMessage =
            match serde_json::from_str(request.get_ref().body.as_str()) {
                Ok(msg) => msg,
                Err(_) => {
                    return Err(Status::failed_precondition(
                        "Error decoding message into a SignedMessage.",
                    ))
                }
            };

        // Now we have a valid message, fetch the session handler
        let session_handle = request.extensions().get::<SessionHandle>().unwrap();
        let mut session = session_handle.write().await;

        // Verify the signed message
        let message = &signed_message.message;
        let session_nonce = match session.get(NONCE_KEY) {
            Some(no) => no,
            None => return Err(Status::unauthenticated("Failed to get nonce.")),
        };

        match message
            .verify(
                signed_message.signature.as_ref(),
                &VerificationOpts {
                    nonce: Some(session_nonce),
                    ..Default::default()
                },
            )
            .await
        {
            Ok(_) => (),
            Err(error) => {
                return Err(Status::unauthenticated(format!(
                    "Invalid signature {:?}.",
                    error
                )))
            }
        }

        // Update the session expiry time and user address
        let now = match unix_timestamp() {
            Ok(now) => now,
            Err(_) => return Err(Status::internal("Failed to get timestamp.")),
        };

        let expiry = now + SEVEN_DAYS_IN_SECONDS;
        match session.insert(EXPIRATION_TIME_KEY, expiry) {
            Ok(_) => (),
            Err(_) => return Err(Status::internal("Failed to insert expiration time.")),
        }

        match session.insert(USER_ADDRESS_KEY, Address::from(message.address)) {
            Ok(_) => (),
            Err(_) => return Err(Status::internal("Failed to insert user address.")),
        }

        Ok(Response::new(Empty {}))
    }

    async fn authenticate(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        let session_handle = request.extensions().get::<SessionHandle>().unwrap();
        let session = session_handle.read().await;
        authenticate(&session).map(|_| Response::new(Empty::default()))
    }
}
