use anyhow::{bail, Result};
use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct ErrorEnvelope {
    pub error: ApiError,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code:       u32,
    pub message:    String,
    #[serde(default)]
    pub data:       Option<serde_json::Value>,
    #[serde(rename = "requestId")]
    pub request_id: String,
}