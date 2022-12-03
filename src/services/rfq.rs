use tonic::{transport::Server, Response, Status};

use crate::request_for_quote::request_for_quote_server::RequestForQuote;
use crate::request_for_quote::QuoteRequest;
use crate::request_for_quote::QuoteResponse;

#[derive(Debug, Default)]
pub struct MyRFQ {}

#[tonic::async_trait]
impl RequestForQuote for MyRFQ {
    async fn request_quote(
        &self,
        request: tonic::Request<QuoteRequest>,
    ) -> Result<Response<QuoteResponse>, Status> {
        println!("Got a request for quote: {:?}", request);

        let reply = QuoteResponse {
            price: format!("Hello $1 for {}!", request.into_inner().token).into(),
        };

        Ok(Response::new(reply))
    }
}
