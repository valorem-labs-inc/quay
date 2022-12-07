use http::Uri;
use quay::rfq::rfq_client::RfqClient;
use quay::rfq::{QuoteRequest, QuoteResponse};
use std::env;
use std::process::exit;
use tokio::sync::mpsc;
use tonic::transport::Channel;

/// An example Market Maker (MM) client interface to Quay.
///
/// The Market Maker will receive Request For Quote (RFQ) from the Quay server formatted as
/// `QuoteRequest` and the MM needs to respond with `QuoteResponse`.
///
/// # Usage
/// `client <quay_server>`
/// where:
/// `<quay_server>`: The location of the Quay server, for example `http://localhost:8000`
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("Unexpected command line arguments. Received {:?}", args);
        eprintln!("Usage: example-client quay-server\nWhere:\n\tquay-server: HTTP location, e.g. http://localhost:8080");
        exit(1);
    }

    if args[0].chars().last().unwrap() != '/' {
        args[0].push('/');
    }

    // TODO: authenticate with Quay

    // Connect to the Quay server's RFQ endpoint
    let rfq_uri = format!("{}rfq", args[0]).parse::<Uri>().unwrap();
    let mut client = RfqClient::new(Channel::builder(rfq_uri).connect().await.unwrap());

    // Setup the comms channels. The RFQ endpoint depends on the client never disconnecting
    // as it uses gRPC bidirectional streams. However the requests we send to the server are
    // actually responses from the server requests (which are responses in the connection).
    //
    // Therefore when we initiate the connection we need to internally loop forever on the server
    // responses which isn't received until we make the connection.
    //
    // As a result we need to setup a channel such that a Tokio task can be sent the
    // responses (actually requests) from the server which it then replies with
    // requests (actually responses).
    //
    // In summary:
    // Server responses are requests to the client. Client requests are responses to the server.
    let (tx_server_response, rx_server_response) = mpsc::channel::<QuoteResponse>(64);
    let (tx_server_request, mut rx_server_request) = mpsc::channel::<QuoteRequest>(64);

    let task = tokio::spawn(async move {
        // Loop until the stream feeding us the server requests ends
        while let Some(request_for_quote) = rx_server_request.recv().await {
            let quote_offer = handle_server_request(request_for_quote);

            // Send the response to the server
            tx_server_response.send(quote_offer).await.unwrap();
        }

        eprintln!("Client connection to the server has been closed");
    });

    // Call the required function which will return the servers response stream (which is really
    // requests to the client).
    let mut quote_stream = client
        .maker(tokio_stream::wrappers::ReceiverStream::new(
            rx_server_response,
        ))
        .await
        .unwrap()
        .into_inner();

    // Now we have received the servers request stream - loop until it ends (its not expected to).
    while let Ok(Some(quote)) = quote_stream.message().await {
        println!("Received quote request from server.");
        tx_server_request.send(quote).await.unwrap();
    }

    // Explicitly drop the tx side of the channel since we know this is the only one to notify
    // the rx side to exit as we are about to await on the task waiting on the rx side.
    drop(tx_server_request);

    // We never expect to get here or the task to end unless the server has disconnected.
    Ok(task.await.unwrap())
}

fn handle_server_request(_request_for_quote: QuoteRequest) -> QuoteResponse {
    QuoteResponse {
        ..Default::default()
    }
}
