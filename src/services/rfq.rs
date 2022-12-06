use crate::rfq::trader_server::Trader;
use crate::rfq::{QuoteResponse, ValoremQuoteRequest};
use std::pin::Pin;
use tokio::sync::mpsc::channel;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct TraderRFQService {}

#[tonic::async_trait]
impl Trader for TraderRFQService {
    type QuoteStream =
        Pin<Box<dyn Stream<Item = Result<ValoremQuoteRequest, Status>> + Send + Sync + 'static>>;

    async fn quote(
        &self,
        request: Request<Streaming<QuoteResponse>>,
    ) -> Result<Response<Self::QuoteStream>, Status> {
        let (tx_request_for_quote, rx_request_for_quote) =
            channel::<Result<ValoremQuoteRequest, Status>>(64);

        let mut client_stream = request.into_inner();

        // This task will send the Quote requests to the MM client that has just connected.
        tokio::spawn(async move {
            loop {
                // TODO: receive quote request from front-end client (redis).

                // Forward the front-end client request onto the MM
                tx_request_for_quote
                    .send(Ok(ValoremQuoteRequest {
                        ..Default::default()
                    }))
                    .await
                    .unwrap();

                // We use the stream as a ping-pong stream, so once we have send a quote request
                // await for the MM response.
                if let Ok(Some(response)) = client_stream.message().await {
                    if response.order.is_none() {
                        // TODO: Inform trader of no offer
                        println!("MM was unable to give FE client an offer.");
                    } else {
                        // TODO: Inform trader of offer
                        println!("Offer from MM.");
                    }
                } else {
                    // We encountered an error with the client connection. Drop the connection.
                    break;
                }
            }
        });

        // Send back the stream to the receive side of the channel that the Tokio task above will
        // write into.
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx_request_for_quote),
        )))
    }
}
