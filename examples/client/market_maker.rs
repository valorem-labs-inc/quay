use ethers::prelude::{Address, LocalWallet, Signer, H160};
use http::Uri;
use quay::rfq;
use quay::rfq::rfq_client::RfqClient;
use quay::rfq::{QuoteRequest, QuoteResponse};
use quay::session::session_client::SessionClient;
use quay::session::{Empty, VerifyText};
use quay::utils::session_interceptor::SessionInterceptor;
use siwe::{TimeStamp, Version};
use std::env;
use std::process::exit;
use std::str::FromStr;
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tonic::transport::Channel;

const SESSION_COOKIE_KEY: &str = "set-cookie";

/// An example Market Maker (MM) client interface to Quay.
///
/// The Market Maker will receive Request For Quote (RFQ) from the Quay server formatted as
/// `QuoteRequest` and the MM needs to respond with `QuoteResponse`.
///
/// # Usage
/// `client <quay_server> <wallet_address>`
/// where:
/// `<quay_server>`     : The location of the Quay server, for example `http://localhost:8000`
/// `<wallet_address>`  : The address of the wallet for signing messages.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        eprintln!("Unexpected command line arguments. Received {args:?}");
        eprintln!("Usage: example-client quay-server wallet_address");
        exit(1);
    }

    let quay_uri = args[0].parse::<Uri>().unwrap();
    let (session_cookie, maker_address) = setup(quay_uri.clone(), args[1].to_string()).await;

    // Now there is a valid authenticated session, connect to the RFQ stream
    let mut client = RfqClient::with_interceptor(
        Channel::builder(quay_uri).connect().await.unwrap(),
        SessionInterceptor { session_cookie },
    );

    // Setup the comms channels. Server responses are requests to the client.
    // Client requests are responses to the server.
    let (tx_quote_response, rx_quote_response) = mpsc::channel::<QuoteResponse>(64);
    let (tx_quote_request, mut rx_quote_request) = mpsc::channel::<QuoteRequest>(64);

    // The main task that handles incoming server requests
    let task = tokio::spawn(async move {
        while let Some(request_for_quote) = rx_quote_request.recv().await {
            let quote_offer = handle_server_request(request_for_quote, maker_address);

            // Send the response to the server
            tx_quote_response.send(quote_offer).await.unwrap();
        }

        eprintln!("Client connection to the server has been closed");
    });

    // Call the required function which will return the servers response stream (which is really
    // requests to the client).
    let mut quote_stream = client
        .maker(tokio_stream::wrappers::ReceiverStream::new(
            rx_quote_response,
        ))
        .await
        .unwrap()
        .into_inner();

    // Now we have received the servers request stream - loop until it ends (its not expected to).
    while let Ok(Some(quote)) = quote_stream.message().await {
        tx_quote_request.send(quote).await.unwrap();
    }

    // Explicitly drop the tx side of the channel allowing the rx side to get notified we are
    // about to close.
    drop(tx_quote_request);

    // We never expect to get here or the task to end unless the server has disconnected.
    task.await.unwrap();
    Ok(())
}

// Handle the quote.
// The current example simply sends back an empty order (indicating no offer).
fn handle_server_request(request_for_quote: QuoteRequest, maker_address: H160) -> QuoteResponse {
    println!("Request received, returning no offer");
    QuoteResponse {
        ulid: request_for_quote.ulid,
        maker_address: Some(rfq::H160::from(maker_address)),
        order: None,
    }
}

// Helper function used to setup a valid session with Quay
async fn setup(quay_uri: Uri, private_key: String) -> (String, Address) {
    // Connect and authenticate with Quay
    let mut client: SessionClient<Channel> =
        SessionClient::new(Channel::builder(quay_uri.clone()).connect().await.unwrap());
    let response = client
        .nonce(Empty::default())
        .await
        .expect("Unable to fetch Nonce from Quay");

    // Fetch the session cookie for all future requests
    let session_cookie = response
        .metadata()
        .get(SESSION_COOKIE_KEY)
        .expect("Session cookie was not returned in Nonce response")
        .to_str()
        .expect("Unable to fetch session cookie from Nonce response")
        .to_string();

    let nonce = response.into_inner().nonce;

    // Verify & authenticate with Quay before connecting to RFQ endpoint.
    let mut client = SessionClient::with_interceptor(
        Channel::builder(quay_uri).connect().await.unwrap(),
        SessionInterceptor {
            session_cookie: session_cookie.clone(),
        },
    );

    // Setup a local wallet
    let wallet = LocalWallet::from_str(private_key.as_str()).unwrap();

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

    // Generate a signature
    let message_string = message.to_string();
    let signature = wallet
        .sign_message(message_string.as_bytes())
        .await
        .unwrap();

    // Create the SignedMessage
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
    let body = serde_json::Value::from(signed_message).to_string();

    // Verify the session with Quay
    let response = client.verify(VerifyText { body }).await;
    match response {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Unable to verify client. Reported error:\n{error:?}");
            exit(2);
        }
    }

    // Check that we have an authenticated session
    let response = client.authenticate(Empty::default()).await;
    match response {
        Ok(_) => (),
        Err(error) => {
            eprintln!(
                "Unable to check authentication with Quay. Reported error:\n{error:?}"
            );
            exit(3);
        }
    }

    (session_cookie, wallet.address())
}
