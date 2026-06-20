use crate::models::Platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub id: u64,
    #[serde(rename = "type")]
    pub req_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub id: u64,
    #[serde(rename = "type")]
    pub resp_type: String,
    pub payload: serde_json::Value,
    pub error: Option<RpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    pub code: u32,
    pub message: String,
}

impl RpcResponse {
    pub fn ok(id: u64, resp_type: impl Into<String>, payload: impl Serialize) -> Self {
        Self {
            id,
            resp_type: resp_type.into(),
            payload: serde_json::to_value(payload).unwrap_or(serde_json::Value::Null),
            error: None,
        }
    }

    pub fn err(id: u64, code: u32, message: impl Into<String>) -> Self {
        Self {
            id,
            resp_type: "Error".into(),
            payload: serde_json::Value::Null,
            error: Some(RpcError {
                code,
                message: message.into(),
            }),
        }
    }
}

// ---- Request payloads ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenProblemRequest {
    pub problem_id: String,
    pub platform: String,
}

// ---- Response payloads ----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDataResponse {
    pub statement: String,
    pub samples: Vec<SampleData>,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapRequest {
    pub handle: String,
    pub year: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleData {
    pub input: String,
    pub output: String,
}

pub fn platform_from_str(s: &str) -> Option<Platform> {
    match s.to_lowercase().as_str() {
        "codeforces" => Some(Platform::Codeforces),
        "cses" => Some(Platform::CSES),
        "atcoder" => Some(Platform::AtCoder),
        _ => None,
    }
}
