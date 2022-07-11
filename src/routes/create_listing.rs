use crate::seaport::Order;
use actix_web::{post, web, HttpResponse, Responder};

// TODO(Implement queries)
// Cleanroom rewrite of: https://docs.opensea.io/v2.0/reference/create-an-order
// Differs a bit from the opensea struct, sticking with rust style key naming
//
// {
//     "parameters": {
//         "offerer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
//         "zone": "0x004c00500000ad104d7dbd00e3ae0a5c00560c00",
//         "zoneHash": "0x3000000000000000000000000000000000000000000000000000000000000000",
//         "startTime": "0",
//         "endTime": "1656044994000",
//         "orderType": 0,
//         "offer": [
//                     {
//                 "itemType": 0,
//                 "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//                 "identifierOrCriteria": "0",
//                 "startAmount": "10000000000000000000",
//                 "endAmount": "10000000000000000000",
//             },
//         ],
//         "consideration": [
//             {
//                 "itemType": 2,
//                 "token": "0x0165878A594ca255338adfa4d48449f69242Eb8F",
//                 "identifierOrCriteria": "1",
//                 "startAmount": "1",
//                 "endAmount": "1",
//                 "recipient": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
//             },
//             {
//                 "itemType": 0,
//                 "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//                 "identifierOrCriteria": "0",
//                 "startAmount": "250000000000000000",
//                 "endAmount": "250000000000000000",
//                 "recipient": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
//             },
//             {
//                 "itemType": 0,
//                 "token": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
//                 "identifierOrCriteria": "0",
//                 "startAmount": "500000000000000000", # Collection Fee
//                 "endAmount": "500000000000000000",
//                 "recipient": "0x8a90cab2b38dba80c64b7734e58ee1db38b8992e",
//             },
//         ],
//         "totalOriginalConsiderationItems": 2,
//         "salt": 12686911856931635052326433555881236148,
//         "conduitKey": "0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000",
//         "nonce": 0,
//     },
//     "signature": "0x",
// }

#[post("/listings")]
async fn create_listing(_form: web::Json<Order>) -> impl Responder {
    HttpResponse::Ok()
}
