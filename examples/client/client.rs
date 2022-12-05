use std::env;
use std::process::exit;
use tokio::sync::mpsc;
use quay::rfq::{QuoteResponse, ValoremQuoteRequest};
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
    let (tx, rx) = mpsc::channel(64);
    let (mtx, mut mrx) = mpsc::channel::<ValoremQuoteRequest>(64);

    tokio::spawn(async move {
        let response = QuoteResponse {
            ..Default::default()
        };
        tx.send(response).await.unwrap();

        while let Some(quote) = mrx.recv().await {
            tx.send(QuoteResponse{
                message_id: quote.message_id,
                ..Default::default()
            }).await.unwrap();
        }
    });

    let mut quotes = client.quote(tokio_stream::wrappers::ReceiverStream::new(rx)).await.unwrap().into_inner();
    while let Some(quote) = quotes.message().await? {
        println!("Quote request {}", quote.message_id);
        mtx.send(quote).await.unwrap();
    }

    Ok(())
}
