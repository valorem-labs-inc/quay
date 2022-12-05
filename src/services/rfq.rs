use crate::rfq::quote_server::Quote;
use crate::rfq::QuoteRequest;
use crate::rfq::QuoteResponse;
use tonic::{Response, Status};

#[derive(Debug, Default)]
pub struct RFQService {}

#[tonic::async_trait]
impl Quote for RFQService {
    async fn quote(
        &self,
        _request: tonic::Request<QuoteRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        Err(Status::unimplemented("Not implemented yet"))
    }
}
