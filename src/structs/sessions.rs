use actix_session::{Session, SessionExt, SessionGetError, SessionInsertError};
use actix_web::cookie::time::OffsetDateTime;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use ethers::abi::ethereum_types::Signature;
use ethers::abi::AbiEncode;
use ethers::prelude::Address;
use serde::{Deserialize, Serialize};
use siwe::Message;
use std::future::{ready, Ready};

// EIP-4361 based session

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedMessage {
    pub signature: Signature,
    pub message: Message,
}

pub struct TypedSession(Session);

impl TypedSession {
    const NONCE_KEY: &'static str = "nonce";
    const EXPIRATION_TIME_KEY: &'static str = "expirationTime";
    const USER_ADDRESS_KEY: &'static str = "userAddress";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_nonce(&self, nonce: &String) -> Result<(), SessionInsertError> {
        self.0.insert(Self::NONCE_KEY, nonce)
    }

    pub fn get_nonce(&self) -> Result<Option<String>, SessionGetError> {
        self.0.get(Self::NONCE_KEY)
    }

    pub fn insert_expiration_time(
        &self,
        expiration_time: &OffsetDateTime,
    ) -> Result<(), SessionInsertError> {
        self.0
            .insert(Self::EXPIRATION_TIME_KEY, expiration_time.unix_timestamp())
    }

    pub fn get_expiration_time(&self) -> Result<Option<OffsetDateTime>, SessionGetError> {
        match self.0.get(Self::EXPIRATION_TIME_KEY) {
            Ok(timestamp) => {
                Ok(timestamp.map(|ts| OffsetDateTime::from_unix_timestamp(ts).unwrap()))
            }
            Err(error) => Err(error),
        }
    }

    pub fn insert_address(&self, address: &Address) -> Result<(), SessionInsertError> {
        self.0.insert(Self::USER_ADDRESS_KEY, address.encode_hex())
    }

    pub fn get_address(&self) -> Result<Option<Address>, SessionGetError> {
        match self.0.get(Self::USER_ADDRESS_KEY) {
            Ok(address) => {
                let addy: Option<Address> = address;
                Ok(addy)
            }
            Err(error) => Err(error),
        }
    }
}

impl FromRequest for TypedSession {
    // This is a complicated way of saying
    // "We return the same error returned by the
    // implementation of `FromRequest` for `Session`".
    type Error = <Session as FromRequest>::Error;
    // Rust does not yet support the `async` syntax in traits.
    // From request expects a `Future` as return type to allow for extractors
    // that need to perform asynchronous operations (e.g. a HTTP call)
    // We do not have a `Future`, because we don't perform any I/O,
    // so we wrap `TypedSession` into `Ready` to convert it into a `Future` that
    // resolves to the wrapped value the first time it's polled by the executor.
    type Future = Ready<Result<TypedSession, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSession(req.get_session())))
    }
}
