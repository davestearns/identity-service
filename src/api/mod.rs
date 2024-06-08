//! The `api` module contains implementations of any APIs this service supports.
//! To start with, we will implement a simple RESTy API, but in the future, we
//! can add other types of APIs such as websockets, gRPC, graphQL, or even SOAP 😱!

pub mod models;
pub mod rest;
pub mod errors;