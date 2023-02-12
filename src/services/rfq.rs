use crate::rfq::rfq_server::Rfq;
use crate::rfq::{QuoteRequest, QuoteResponse, H128};
use ethers::prelude::U128;
use std::collections::HashMap;
use std::pin::Pin;
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::channel;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status, Streaming};
use ulid::Ulid;

#[derive(Debug)]
pub struct RFQService {
    request_tx_stream: Sender<QuoteRequest>,
    response_tx_stream: Sender<QuoteResponse>,
}

impl RFQService {
    pub fn new() -> Self {
        let (request_tx_stream, _) = tokio::sync::broadcast::channel::<QuoteRequest>(64);
        let (response_tx_stream, _) = tokio::sync::broadcast::channel::<QuoteResponse>(64);
        Self {
            request_tx_stream,
            response_tx_stream,
        }
    }
}

impl Default for RFQService {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl Rfq for RFQService {
    type WebTakerStream =
        Pin<Box<dyn Stream<Item = Result<QuoteResponse, Status>> + Send + 'static>>;

    async fn web_taker(
        &self,
        request: Request<QuoteRequest>,
    ) -> Result<Response<Self::WebTakerStream>, Status> {
        // Create the stream which will be sent back to the web taker (trader). This will only
        // hold one item at max.
        let (tx_trader, rx_trader) = channel::<Result<QuoteResponse, Status>>(1);

        // Insert the ulid into the messages so we are able to identify a response to this request.
        let ulid: U128 = Ulid::new().0.into();
        let mut request = request.into_inner();
        request.ulid = Some(ulid.into());

        // Send the request off to the makers
        match self.request_tx_stream.send(request) {
            Ok(_) => {
                let mut response_broadcast = self.response_tx_stream.subscribe();
                let ulid: Option<H128> = Some(ulid.into());

                // Using a tokio task wait until we get a response from the makers and if it
                // matches our ulid forward it onto the taker.
                tokio::spawn(async move {
                    let mut end_stream = false;
                    while !end_stream {
                        match response_broadcast.recv().await {
                            Ok(response) => {
                                if ulid == response.ulid {
                                    tx_trader.send(Ok(response)).await.unwrap_or_default();
                                    end_stream = true;
                                }
                            }
                            Err(error) => {
                                eprintln!("RFQService:WebTaker: Error while receiving broadcast requests. Error reported\n{error}");
                                tx_trader
                                    .send(Err(Status::internal("Internal server error 2")))
                                    .await
                                    .unwrap_or_default();
                                end_stream = true;
                            }
                        }
                    }

                    drop(response_broadcast);
                });
            }
            Err(error) => {
                eprintln!(
                    "RFQService:WebTaker: Error while broadcasting request. Error reported\n{error}"
                );
                tx_trader
                    .send(Err(Status::internal("Internal server error 1")))
                    .await
                    .unwrap_or_default();
            }
        }

        // Response stream for the taker to wait on
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx_trader),
        )))
    }

    type TakerStream = Pin<Box<dyn Stream<Item = Result<QuoteResponse, Status>> + Send + 'static>>;

    async fn taker(
        &self,
        request: Request<Streaming<QuoteRequest>>,
    ) -> Result<Response<Self::TakerStream>, Status> {
        // Setup the private channel between the Tokio task and the connected Taker. This buffer
        // can hold 64 messages before it will block on the `await` waiting for a message to be
        // removed.
        let (tx_taker, rx_taker) = channel::<Result<QuoteResponse, Status>>(64);

        let mut taker_stream: Streaming<QuoteRequest> = request.into_inner();
        let mut response_rx_stream = self.response_tx_stream.subscribe();
        let request_tx_stream = self.request_tx_stream.clone();

        // This task will pass messages to and from a connected taker.
        tokio::spawn(async move {
            // Loop forever until we hit one of the break conditions (i.e. a stream has died).
            let mut stream_closed = false;
            let mut message_ids = HashMap::<U128, ()>::new();

            while !stream_closed {
                // Wait until one of the streams return.
                tokio::select! {
                    request = taker_stream.message() => {
                        match request {
                            Ok(request) => {
                                match request {
                                    Some(request) => {
                                        // Insert the id for this message
                                        let ulid: U128 = Ulid::new().0.into();
                                        let mut request = request;
                                        request.ulid = Some(ulid.into());

                                        message_ids.insert(ulid, ());
                                        request_tx_stream.send(request).unwrap_or_default();
                                    },
                                    None => {
                                        eprintln!("RFQService:Taker: Taker stream as closed.");
                                        stream_closed = true;
                                    }
                                }
                            },
                            Err(error) => {
                                eprintln!("RFQService:Taker: Error while handling taker stream. Reported error\n{error}");
                                stream_closed = true;
                            }
                        }
                    },
                    response = response_rx_stream.recv() => {
                        match response {
                            Ok(response) => {
                                if let Some(ulid) = response.ulid.clone() {
                                    let ulid: U128 = ulid.into();
                                    if message_ids.contains_key(&ulid) {
                                        // Send the request to the taker
                                        tx_taker.send(Ok(response)).await.unwrap_or_default();
                                        message_ids.remove(&ulid);
                                    }
                                }
                            },
                            Err(error) => {
                                eprintln!("RFQService:Taker: Error while reading response broadcast stream. Reported error\n{error}");
                                tx_taker.send(Err(Status::internal("Internal server error 3"))).await.unwrap_or_default();
                                stream_closed = true;
                            }
                        }
                    }
                }
            }
        });

        // Send back the stream to the receive side of the channel that the Tokio task above will
        // write into.
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx_taker),
        )))
    }

    type MakerStream = Pin<Box<dyn Stream<Item = Result<QuoteRequest, Status>> + Send + 'static>>;

    /// When a Maker makes an initial connection, spawn a asynchronous tasks that will await on
    /// either a new request from a Trader/Taker (to then forward to the Maker) or a response from
    /// the Maker (to then forward onto the Trader/Taker).
    async fn maker(
        &self,
        request: Request<Streaming<QuoteResponse>>,
    ) -> Result<Response<Self::MakerStream>, Status> {
        // Setup the private channel between the Tokio task and the connected Maker. This buffer
        // can hold 64 messages before it will block on the `await` waiting for a message to be
        // removed.
        let (tx_maker, rx_maker) = channel::<Result<QuoteRequest, Status>>(64);

        let mut maker_stream: Streaming<QuoteResponse> = request.into_inner();
        let mut request_rx_stream = self.request_tx_stream.subscribe();
        let response_tx_stream = self.response_tx_stream.clone();

        // This task will pass messages to and from a connected Maker.
        tokio::spawn(async move {
            // Loop forever until we hit one of the break conditions (i.e. a stream has died).
            let mut stream_closed = false;
            while !stream_closed {
                // Wait until one of the streams return.
                tokio::select! {
                    request = request_rx_stream.recv() => {
                        // Received a new request - forward to Maker or exit if error
                        match request {
                            Ok(request) => tx_maker.send(Ok(request)).await.unwrap(),
                            Err(error) => {
                                eprintln!("RFQService:Maker: Request stream has closed. Reported error\n{error:?}");
                                tx_maker.send(Err(Status::internal("Internal server error 4"))).await.unwrap_or_default();
                                stream_closed = true;
                            }
                        }
                    },
                    response = maker_stream.message() => {
                        // Received a new response - forward to takers/traders or exit if error
                        match response {
                            Ok(Some(response)) => {
                                match response_tx_stream.send(response) {
                                    Ok(_) => (),
                                    Err(error) => {
                                        eprintln!("RFQService:Maker: All response tx stream receivers have been dropped. Error reported\n{error}");
                                        tx_maker.send(Err(Status::internal("Internal server error 5"))).await.unwrap_or_default();
                                        stream_closed = true;
                                    }
                                }
                            },
                            Ok(None) => {
                                eprintln!("RFQService:Maker: Stream to maker has been closed.");
                                stream_closed = true;
                            },
                            Err(error) => {
                                eprintln!("RFQService:Maker: Error while handling maker stream. Reported error\n{error:?}");
                                stream_closed = true;
                            }
                        }
                    }
                }
            }

            // Drop the transmit stream to allow the receive side to detect we are exiting.
            drop(tx_maker);
        });

        // Send back the stream to the receive side of the channel that the Tokio task above will
        // write into.
        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx_maker),
        )))
    }
}
