use reqwest::{header::HeaderMap, Method};
use serde_json::Value;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    FailedToFetch(#[from] reqwest::Error),

    #[error("Failed to parse response of {0}:{1}")]
    FailedToParseResponse(Method, String),
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Value,
}

// Response for request
#[derive(Debug, Clone)]
pub struct Response {
    pub status: reqwest::StatusCode,
    pub data: String,
}

impl Response {
    pub fn new(status: reqwest::StatusCode, data: String) -> Self {
        Self { status, data }
    }
}
