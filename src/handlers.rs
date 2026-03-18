use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::store::MessageStore;
use crate::models::{MessagesResponse, ProjectInfo, SessionInfo, Stats};

#[derive(Debug, Deserialize)]
pub struct MessagesQuery {
    pub project: Option<String>,
    pub session: Option<String>,
    pub q: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    #[serde(default = "default_offset")]
    pub offset: usize,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_offset() -> usize {
    0
}
fn default_limit() -> usize {
    100
}

#[derive(Debug, Deserialize)]
pub struct SessionsQuery {
    pub project: Option<String>,
}

const MAX_LIMIT: usize = 500;

pub async fn get_messages(
    State(store): State<MessageStore>,
    Query(q): Query<MessagesQuery>,
) -> Json<MessagesResponse> {
    let resp = store.query_messages(
        q.project.as_deref(),
        q.session.as_deref(),
        q.q.as_deref(),
        q.from,
        q.to,
        q.offset,
        q.limit.min(MAX_LIMIT),
    );
    Json(resp)
}

pub async fn get_projects(State(store): State<MessageStore>) -> Json<Vec<ProjectInfo>> {
    Json(store.projects())
}

pub async fn get_sessions(
    State(store): State<MessageStore>,
    Query(q): Query<SessionsQuery>,
) -> Json<Vec<SessionInfo>> {
    Json(store.sessions(q.project.as_deref()))
}

pub async fn get_stats(State(store): State<MessageStore>) -> Json<Stats> {
    Json(store.stats())
}
