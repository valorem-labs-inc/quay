use crate::request_for_quote::quote_server::Quote;
use crate::request_for_quote::QuoteRequest;
use crate::request_for_quote::QuoteResponse;
use tonic::{Response, Status};

#[derive(Debug, Default)]
pub struct RFQService {}

#[tonic::async_trait]
impl Quote for RFQService {
    async fn quote(
        &self,
        request: tonic::Request<QuoteRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        println!("Got a request for quote: Url: {}", request.into_inner().url);

        let reply = QuoteResponse {
            confirmation: "HelloWorld from QuoteRequest".to_string(),
        };

        Ok(Response::new(reply))
    }
}
