use std::env;
use std::process::exit;
use futures::stream;
use tonic::Request;
use quay::rfq::QuoteResponse;
use quay::rfq::trader_client::TraderClient;

/// An example Market Maker (MM) client interface to Quay.
///
/// The Market Maker will receive Request For Quote (RFQ) formatted as `QuoteRequest` and response
/// with `QuoteResponse`.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Take a single command line argument of the location of the Quay server.
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Unexpected command line arguments. Received {:?}", args);
        eprintln!("Usage: example-client quay-server\nWhere:\n\tquay-server: HTTP location, e.g. http://localhost:8080");
        exit(1);
    }

    let channel = tonic::transport::Channel::from_static("http://localhost:8000/rfq")
        .connect()
        .await
        .unwrap();

    let mut client = TraderClient::new(channel);

    // Make an request for any quotes
    let response = QuoteResponse {
        ..Default::default()
    };

    let response = Request::new(stream::iter(vec![response]));
    let mut quotes = client.quote(response).await.unwrap().into_inner();

    while let Some(quote) = quotes.message().await? {
        println!("Quote request {}", quote.message_id)
    }

    Ok(())
}
