use crate::helpers::spawn_app;
extern crate serde_json;

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[actix_rt::test]
async fn create_listing_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

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
                "totalOriginalConsiderationItems": "2",
                "salt": "12686911856931635052326433555881236148",
                "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
                "nonce": 0
            },
            "signature": "0x"
        }
    "#;

    let json_body: serde_json::Value = serde_json::from_str(listing_file).expect("bad test file");

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/listings", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
