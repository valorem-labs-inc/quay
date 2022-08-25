use crate::helpers::spawn_app;
use quay::{seaport::Order, structs::RetrieveResponse};
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
async fn create_and_retrieve_offer_works() {
    // Arrange
    let app = spawn_app().await;

    // This pffer sample came from the original API spec with the keys modified to match the rust
    // struct keys.
    let offer_file = r#"
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
                        "itemType": 0,
                        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                        "identifierOrCriteria": "0",
                        "startAmount": "10000000000000000000",
                        "endAmount": "10000000000000000000"
                    }
                ],
                "consideration": [
                    {
                        "itemType": 2,
                        "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
                        "identifierOrCriteria": "1",
                        "startAmount": "1",
                        "endAmount": "1",
                        "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
                    },
                    {
                        "itemType": 0,
                        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
                        "identifierOrCriteria": "0",
                        "startAmount": "250000000000000000",
                        "endAmount": "250000000000000000",
                        "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8"
                    },
                    {
                        "itemType": 0,
                        "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
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

    let json_body: Order = serde_json::from_str(offer_file).expect("bad test file");

    // Act
    let create_response = app
        .api
        // Use the returned application address
        .post(&format!("{}/offers", &app.address))
        .json(&json_body)
        .send()
        .await
        .expect("Failed to execute create request.");

    // Assert
    assert!(create_response.status().is_success());

    let retrieve_response = app
        .api
        // Use the returned application address
        .get(&format!("{}/offers?asset_contract_address=0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2&token_ids=0", &app.address))
        .send()
        .await
        .expect("Failed to execute retrieve request.")
        .json::<RetrieveResponse>()
        .await
        .expect("Failed to get retrieve request json result.");

    let first_order = retrieve_response
        .orders
        .first()
        .expect("There should be at least 1 order.")
        .protocol_data
        .clone();

    assert!(first_order == json_body);
}
