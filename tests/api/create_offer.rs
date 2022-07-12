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
async fn create_offer_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // This listing sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let offer_file = r#"{
  "parameters": {
    "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    "zone": "0x004c00500000ad104d7dbd00e3ae0a5c00560c00",
    "zone_hash": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    "start_time": "0",
    "end_time": "1656044994000",
    "order_type": 0,
    "offer": [
      {
        "item_type": 0,
        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "identifier_or_criteria": "0",
        "start_amount": "10000000000000000000",
        "end_amount": "10000000000000000000"
      }
    ],
    "consideration": [
      {
        "item_type": 2,
        "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
        "identifier_or_criteria": "1",
        "start_amount": "1",
        "end_amount": "1",
        "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
      },
      {
        "item_type": 0,
        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "identifier_or_criteria": "0",
        "start_amount": "250000000000000000",
        "end_amount": "250000000000000000",
        "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
      },
      {
        "item_type": 0,
        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        "identifier_or_criteria": "0",
        "start_amount": "500000000000000000",
        "end_amount": "500000000000000000",
        "recipient": "0x8a90cab2b38dba80c64b7734e58ee1db38b8992e"
      }
    ],
    "total_original_consideration_items": "2",
    "salt": "12686911856931635052326433555881236148",
    "conduit_key": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    "nonce": 0
  },
  "signature": "0x"
}"#;

    let json_body: serde_json::Value = serde_json::from_str(offer_file).expect("bad test file");

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/offers", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
