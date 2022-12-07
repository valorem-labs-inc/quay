use crate::rfq::rfq_server::Rfq;
use crate::rfq::{QuoteRequest, QuoteResponse};
use std::pin::Pin;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Default)]
pub struct RFQService {}

#[tonic::async_trait]
impl Rfq for RFQService {
    type WebTakerStream =
        Pin<Box<dyn Stream<Item = Result<QuoteResponse, Status>> + Send + 'static>>;

    async fn web_taker(
        &self,
        _request: Request<QuoteRequest>,
    ) -> Result<Response<Self::WebTakerStream>, Status> {
        todo!("Implement me")
    }

    type TakerStream = Pin<Box<dyn Stream<Item = Result<QuoteResponse, Status>> + Send + 'static>>;

    async fn taker(
        &self,
        _request: Request<Streaming<QuoteRequest>>,
    ) -> Result<Response<Self::TakerStream>, Status> {
        todo!("Implement me")
    }

    type MakerStream = Pin<Box<dyn Stream<Item = Result<QuoteRequest, Status>> + Send + 'static>>;

    async fn maker(
        &self,
        _request: Request<Streaming<QuoteResponse>>,
    ) -> Result<Response<Self::MakerStream>, Status> {
        todo!("Implement me")
    }
}
