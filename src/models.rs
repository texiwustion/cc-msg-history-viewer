use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Raw record from history.jsonl
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub display: String,
    #[serde(default)]
    pub pasted_contents: HashMap<String, serde_json::Value>,
    pub timestamp: i64,
    pub project: String,
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct MessagesResponse {
    pub total: usize,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
pub struct ProjectInfo {
    pub path: String,
    pub count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub session_id: String,
    pub project: String,
    pub first_ts: i64,
    pub count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_messages: usize,
    pub total_projects: usize,
    pub total_sessions: usize,
    pub earliest_ts: Option<i64>,
    pub latest_ts: Option<i64>,
    /// date (YYYY-MM-DD) → count
    pub daily_counts: Vec<DailyCount>,
}

#[derive(Debug, Serialize)]
pub struct DailyCount {
    pub date: String,
    pub count: usize,
}
