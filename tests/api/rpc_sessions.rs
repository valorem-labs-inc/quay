use crate::helpers::spawn_app;
use ethers::signers::{LocalWallet, Signer};
use http::Uri;
use quay::session::session_client::SessionClient;
use quay::session::{Empty, VerifyText};
use quay::utils::session_interceptor::SessionInterceptor;
use siwe::{TimeStamp, Version};
use std::str::FromStr;
use time::OffsetDateTime;
use tonic::transport::Channel;

const SESSION_COOKIE_KEY: &str = "set-cookie";

#[tokio::test]
async fn verify_session_works() {
    // Arrange
    let app = spawn_app().await;

    let mut client = SessionClient::new(
        Channel::builder(app.address.parse::<Uri>().unwrap())
            .connect()
            .await
            .unwrap(),
    );

    // Get the session nonce
    let nonce_response = client.nonce(Empty::default()).await;

    // Assert we got a nonce back
    assert!(nonce_response.is_ok());
    let nonce_response = nonce_response.unwrap();

    // Fetch the session details and nonce.
    let session_cookie = nonce_response
        .metadata()
        .get(SESSION_COOKIE_KEY)
        .expect("Session cookie was not returned in Nonce response")
        .to_str()
        .expect("Unable to fetch session cookie from Nonce response")
        .to_string();

    let nonce = nonce_response.unwrap().into_inner().nonce;

    // Setup the session client with our newly created session.
    let mut client = SessionClient::with_interceptor(
        Channel::builder(app.address.parse::<Uri>().unwrap())
            .connect()
            .await
            .unwrap(),
        SessionInterceptor { session_cookie },
    );

    // Setup a local wallet
    let wallet =
        LocalWallet::from_str("380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc")
            .unwrap();

    // Create a sign in with ethereum message
    let message = siwe::Message {
        domain: "localhost.com".parse().unwrap(),
        address: wallet.address().0,
        statement: None,
        uri: "http://localhost/".parse().unwrap(),
        version: Version::V1,
        chain_id: 1,
        nonce,
        issued_at: TimeStamp::from(OffsetDateTime::now_utc()),
        expiration_time: None,
        not_before: None,
        request_id: None,
        resources: vec![],
    };

    let message_string = message.to_string();

    // Generate a signature
    let signature = wallet
        .sign_message(message_string.as_bytes())
        .await
        .unwrap();

    // Get a string version of the signature
    let signature_string = signature.to_string();

    let mut signed_message = serde_json::Map::new();
    signed_message.insert(
        "signature".to_string(),
        serde_json::Value::from(signature_string),
    );
    signed_message.insert(
        "message".to_string(),
        serde_json::Value::from(message_string),
    );

    let json_body = serde_json::Value::from(signed_message);

    // Act
    let response = client
        .verify(VerifyText {
            body: json_body.to_string(),
        })
        .await;

    // Assert
    assert!(response.is_ok());

    // Check that we have an authenticated session
    let response = client.authenticate(Empty::default()).await;

    // Assert
    assert!(response.is_ok());
}
