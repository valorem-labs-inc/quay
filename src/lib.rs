pub mod bindings;
pub mod configuration;
pub mod indexer;
pub mod routes;
pub mod services;
pub mod startup;
pub mod structs;
pub mod telemetry;

pub mod request_for_quote {
    #![allow(clippy::all)]
    tonic::include_proto!("rfq");
}
