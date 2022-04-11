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

// This struct is sent to http_request
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
    pub fn new(status: reqwest::StatusCode, data: &str) -> Self {
        Self {
            status,
            data: data.to_string(),
        }
    }
}

// ParsedRequest: Data for request
#[derive(Debug, Clone)]
pub struct DataRequest {
    pub name: String,
    pub request: Request,
    pub show_error: bool,
    pub show_output: bool,
    pub show_status: bool,
    pub show_time: bool,
}
