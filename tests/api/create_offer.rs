use crate::helpers::spawn_app;
extern crate serde_json;

use serde_json::{Map, Number, Value};

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

    // TODO(Should this be a serialized order object)
    // This listing sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let mut listing = Map::new();
    let mut parameters = Map::new();
    parameters.insert(
        "offerer".to_string(),
        Value::String("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string()),
    );
    parameters.insert(
        "zone".to_string(),
        Value::String("0x004c00500000ad104d7dbd00e3ae0a5c00560c00".to_string()),
    );
    parameters.insert(
        "zone_hash".to_string(),
        Value::String(
            "0x3000000000000000000000000000000000000000000000000000000000000000".to_string(),
        ),
    );
    parameters.insert("start_time".to_string(), Value::String("0".to_string()));
    parameters.insert(
        "end_time".to_string(),
        Value::String("1656044994000".to_string()),
    );
    parameters.insert("order_type".to_string(), Value::Number(Number::from(0)));
    let mut offer = Map::new();
    offer.insert("item_type".to_string(), Value::Number(Number::from(0)));
    offer.insert(
        "token".to_string(),
        Value::String("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string()),
    );
    offer.insert(
        "identifier_or_criteria".to_string(),
        Value::String("0".to_string()),
    );
    offer.insert(
        "start_amount".to_string(),
        Value::String("10000000000000000000".to_string()),
    );
    offer.insert(
        "end_amount".to_string(),
        Value::String("10000000000000000000".to_string()),
    );
    parameters.insert(
        "offer".to_string(),
        Value::Array(vec![Value::Object(offer)]),
    );
    let mut consideration_zero = Map::new();
    consideration_zero.insert("item_type".to_string(), Value::Number(Number::from(2)));
    consideration_zero.insert(
        "token".to_string(),
        Value::String("0x0165878A594ca255338adfa4d48449f69242Eb8F".to_string()),
    );
    consideration_zero.insert(
        "identifier_or_criteria".to_string(),
        Value::String("1".to_string()),
    );
    consideration_zero.insert("start_amount".to_string(), Value::String("1".to_string()));
    consideration_zero.insert("end_amount".to_string(), Value::String("1".to_string()));
    let mut consideration_one = Map::new();
    consideration_one.insert("item_type".to_string(), Value::Number(Number::from(2)));
    consideration_one.insert(
        "token".to_string(),
        Value::String("0x0165878A594ca255338adfa4d48449f69242Eb8F".to_string()),
    );
    consideration_one.insert(
        "identifier_or_criteria".to_string(),
        Value::String("1".to_string()),
    );
    consideration_one.insert("start_amount".to_string(), Value::String("1".to_string()));
    consideration_one.insert("end_amount".to_string(), Value::String("1".to_string()));
    consideration_one.insert(
        "recipient".to_string(),
        Value::String("0x70997970C51812dc3A010C7d01b50e0d17dc79C8".to_string()),
    );
    let mut consideration_two = Map::new();
    consideration_two.insert("item_type".to_string(), Value::Number(Number::from(0)));
    consideration_two.insert(
        "token".to_string(),
        Value::String("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2".to_string()),
    );
    consideration_two.insert(
        "identifier_or_criteria".to_string(),
        Value::String("0".to_string()),
    );
    consideration_two.insert(
        "start_amount".to_string(),
        Value::String("500000000000000000".to_string()),
    );
    consideration_two.insert(
        "end_amount".to_string(),
        Value::String("500000000000000000".to_string()),
    );
    consideration_two.insert(
        "recipient".to_string(),
        Value::String("0x8a90cab2b38dba80c64b7734e58ee1db38b8992e".to_string()),
    );
    parameters.insert(
        "consideration".to_string(),
        Value::Array(vec![
            Value::Object(consideration_zero),
            Value::Object(consideration_one),
            Value::Object(consideration_two),
        ]),
    );
    parameters.insert(
        "total_original_consideration_items".to_string(),
        Value::Number(Number::from(2)),
    );
    parameters.insert(
        "salt".to_string(),
        Value::Number(Number::from(1268691185693_i64)),
    );
    parameters.insert(
        "conduit_key".to_string(),
        Value::String(
            "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000".to_string(),
        ),
    );
    parameters.insert("nonce".to_string(), Value::Number(Number::from(0)));
    listing.insert("parameters".to_string(), Value::Object(parameters));
    listing.insert("signature".to_string(), Value::String("0x".to_string()));

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/create_offer", &app.address))
        .json(&listing)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
