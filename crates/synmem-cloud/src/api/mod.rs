//! API gateway and routes for SynMem Cloud

pub mod error;
pub mod gateway;
pub mod middleware;
pub mod routes;

pub use error::ApiError;
pub use gateway::ApiGateway;
