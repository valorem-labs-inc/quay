use crate::helpers::spawn_app;
use ethers::signers::{LocalWallet, Signer};
use siwe::{TimeStamp, Version};
use std::str::FromStr;
use time::OffsetDateTime;

#[tokio::test]
async fn verify_session_works() {
    // Arrange
    let app = spawn_app().await;

    // Get the session nonce
    let nonce_response = app
        .api
        // Use the returned application address
        .get(&format!("{}/nonce", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert we got a nonce back
    assert!(nonce_response.status().is_success());

    //let cookie = nonce_response.headers().get("set-cookie").unwrap().clone();
    let nonce = nonce_response.text().await.unwrap();

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
    let response = app
        .api
        // Use the returned application address
        .post(&format!("{}/verify", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    // Check that we have an authenticated session
    let response = app
        .api
        // Use the returned application address
        .get(&format!("{}/authenticate", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
