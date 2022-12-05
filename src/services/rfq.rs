use std::pin::Pin;
use tokio::sync::mpsc::{channel};
use tonic::{Request, Response, Status, Streaming};
use tonic::codegen::futures_core::Stream;
use crate::rfq::{QuoteResponse, ValoremQuoteRequest};
use crate::rfq::trader_server::Trader;

#[derive(Debug, Default)]
pub struct RFQService {}

#[tonic::async_trait]
impl Trader for RFQService {
    type QuoteStream = Pin<Box<dyn Stream<Item = Result<ValoremQuoteRequest, Status>> + Send + Sync + 'static>>;

    // Basic stream setup to send 10 numbers, before closing as a simple client test
    async fn quote(&self, request: Request<Streaming<QuoteResponse>>) -> Result<Response<Self::QuoteStream>, Status> {
        let (tx, rx) = channel(64);

        // Consume the first message - this will always be blank
        let mut client_stream = request.into_inner();
        if let Ok(msg) = client_stream.message().await {
            println!("Received: {}", msg.unwrap().message_id);
        }

        tokio::spawn(async move {
            let mut a = 0u64;
            loop {
                println!("Server sending {}", a);
                tx.send(Ok(ValoremQuoteRequest {
                    message_id: a,
                    ..Default::default()
                }))
                    .await
                    .unwrap();

                a += 1;

                if let Ok(msg) = client_stream.message().await {
                    println!("Server received: {}", msg.unwrap().message_id);
                }
            }
        });

        Ok(Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))))
    }
}
