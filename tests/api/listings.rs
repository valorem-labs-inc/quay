use crate::helpers::spawn_app;
use ethers::prelude::{LocalWallet, Signer};
use quay::structs::OrderInput;
use quay::structs::RetrieveResponse;
use siwe::{TimeStamp, Version};
use std::str::FromStr;
use time::OffsetDateTime;

extern crate serde_json;

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[tokio::test]
async fn create_and_retrieve_listing_works() {
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

    // This listing sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let listing_file = r#"
        {
            "parameters": {
                "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                "zone": "0x004c00500000ad104d7dbd00e3ae0a5c00560c00",
                "zoneHash": "0x3000000000000000000000000000000000000000000000000000000000000000",
                "startTime": "0",
                "endTime": "1656044994000",
                "orderType": 0,
                "offer": [
                    {
                        "itemType": 2,
                        "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
                        "identifierOrCriteria": "1",
                        "startAmount": "1",
                        "endAmount": "1"
                    }
                ],
                "consideration": [
                    {
                        "itemType": 0,
                        "token": "0x0000000000000000000000000000000000000000",
                        "identifierOrCriteria": "0",
                        "startAmount": "9750000000000000000",
                        "endAmount": "9750000000000000000",
                        "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
                    },
                    {
                        "itemType": 0,
                        "token": "0x0000000000000000000000000000000000000000",
                        "identifierOrCriteria": "0",
                        "startAmount": "250000000000000000",
                        "endAmount": "250000000000000000",
                        "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
                    },
                    {
                        "itemType": 0,
                        "token": "0x0000000000000000000000000000000000000000",
                        "identifierOrCriteria": "0",
                        "startAmount": "500000000000000000",
                        "endAmount": "500000000000000000",
                        "recipient": "0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"
                    }
                ],
                "totalOriginalConsiderationItems": 2,
                "salt": "12686911856931635052326433555881236148",
                "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
                "nonce": 0
            },
            "signature": "0x"
        }
    "#;
    let json_body: OrderInput = serde_json::from_str(listing_file).expect("bad test file");

    // Act
    let create_response = app
        .api
        // Use the returned application address
        .post(&format!("{}/listings", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute creation request.");

    // Assert
    assert!(create_response.status().is_success());

    let retrieve_response = app
        .api
        // Use the returned application address
        .get(&format!("{}/listings?asset_contract_address=0x0165878A594ca255338adfa4d48449f69242Eb8F&token_ids=1", &app.address))
        .send()
        .await
        .expect("Failed to execute retrieve request.")
        .json::<RetrieveResponse>()
        .await
        .expect("Failed to get retrieve request json result.");

    let _first_order = retrieve_response
        .orders
        .first()
        .expect("There should be at least 1 order.")
        .protocol_data
        .clone();

    // assert_eq!(first_order, json_body);
}

#[tokio::test]
async fn retrieve_listing_by_contract_address_works() {
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

    // This listing sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let listing_file = r#"
            {
                "parameters": {
                    "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                    "zone": "0x004c00500000ad104d7dbd00e3ae0a5c00560c00",
                    "zoneHash": "0x3000000000000000000000000000000000000000000000000000000000000000",
                    "startTime": "0",
                    "endTime": "1656044994000",
                    "orderType": 0,
                    "offer": [
                        {
                            "itemType": 2,
                            "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
                            "identifierOrCriteria": "1",
                            "startAmount": "1",
                            "endAmount": "1"
                        }
                    ],
                    "consideration": [
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "9750000000000000000",
                            "endAmount": "9750000000000000000",
                            "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
                        },
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "250000000000000000",
                            "endAmount": "250000000000000000",
                            "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
                        },
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "500000000000000000",
                            "endAmount": "500000000000000000",
                            "recipient": "0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"
                        }
                    ],
                    "totalOriginalConsiderationItems": 2,
                    "salt": "12686911856931635052326433555881236148",
                    "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
                    "nonce": 0
                },
                "signature": "0x"
            }
        "#;
    let json_body: OrderInput = serde_json::from_str(listing_file).expect("bad test file");

    // Act
    let create_response = app
        .api
        // Use the returned application address
        .post(&format!("{}/listings", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute creation request.");

    // Assert
    assert!(create_response.status().is_success());

    let retrieve_response = app
        .api
        // Use the returned application address
        .get(&format!(
            "{}/listings?asset_contract_address=0x0165878A594ca255338adfa4d48449f69242Eb8F",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute retrieve request.")
        .json::<RetrieveResponse>()
        .await
        .expect("Failed to get retrieve request json result.");

    let _first_order = retrieve_response
        .orders
        .first()
        .expect("There should be at least 1 order.")
        .protocol_data
        .clone();
}

#[tokio::test]
async fn retrieve_listing_by_offerer_works() {
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

    // This listing sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let listing_file = r#"
            {
                "parameters": {
                    "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
                    "zone": "0x004c00500000ad104d7dbd00e3ae0a5c00560c00",
                    "zoneHash": "0x3000000000000000000000000000000000000000000000000000000000000000",
                    "startTime": "0",
                    "endTime": "1656044994000",
                    "orderType": 0,
                    "offer": [
                        {
                            "itemType": 2,
                            "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
                            "identifierOrCriteria": "1",
                            "startAmount": "1",
                            "endAmount": "1"
                        }
                    ],
                    "consideration": [
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "9750000000000000000",
                            "endAmount": "9750000000000000000",
                            "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
                        },
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "250000000000000000",
                            "endAmount": "250000000000000000",
                            "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
                        },
                        {
                            "itemType": 0,
                            "token": "0x0000000000000000000000000000000000000000",
                            "identifierOrCriteria": "0",
                            "startAmount": "500000000000000000",
                            "endAmount": "500000000000000000",
                            "recipient": "0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"
                        }
                    ],
                    "totalOriginalConsiderationItems": 2,
                    "salt": "12686911856931635052326433555881236148",
                    "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
                    "nonce": 0
                },
                "signature": "0x"
            }
        "#;
    let json_body: OrderInput = serde_json::from_str(listing_file).expect("bad test file");

    // Act
    let create_response = app
        .api
        // Use the returned application address
        .post(&format!("{}/listings", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute creation request.");

    // Assert
    assert!(create_response.status().is_success());

    let retrieve_response = app
        .api
        // Use the returned application address
        .get(&format!(
            "{}/listings?offerer=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute retrieve request.")
        .json::<RetrieveResponse>()
        .await
        .expect("Failed to get retrieve request json result.");

    let _first_order = retrieve_response
        .orders
        .first()
        .expect("There should be at least 1 order.")
        .protocol_data
        .clone();
}
