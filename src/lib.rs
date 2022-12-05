pub mod bindings;
pub mod configuration;
pub mod indexer;
pub mod middleware;
pub mod redis;

pub mod rfq {
    tonic::include_proto!("quote");
}

pub mod routes;
pub mod services;
pub mod startup;
pub mod state;
pub mod structs;
pub mod telemetry;
pub mod utils;

pub mod request_for_quote {
    #![allow(clippy::all)]
    tonic::include_proto!("rfq");
}
