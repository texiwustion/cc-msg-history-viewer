use std::collections::{BTreeMap, HashMap};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use chrono::{TimeZone, Utc};

use crate::models::{DailyCount, Message, MessagesResponse, ProjectInfo, SessionInfo, Stats};

/// Maximum number of messages kept in memory (most-recent wins after sort).
const MAX_MESSAGES: usize = 50_000;

#[derive(Clone)]
pub struct MessageStore {
    messages: Arc<Vec<Message>>,
}

impl MessageStore {
    pub fn load(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut messages = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match serde_json::from_str::<Message>(line) {
                Ok(msg) => messages.push(msg),
                Err(e) => {
                    tracing::warn!("Skipping unparseable line: {e}");
                }
            }
        }
        // Sort by timestamp descending (newest first), then cap
        messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        messages.truncate(MAX_MESSAGES);
        tracing::info!("Loaded {} messages", messages.len());
        Ok(Self {
            messages: Arc::new(messages),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn query_messages(
        &self,
        project: Option<&str>,
        session: Option<&str>,
        q: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
        offset: usize,
        limit: usize,
    ) -> MessagesResponse {
        let q_lower = q.map(|s| s.to_lowercase());

        let filtered: Vec<&Message> = self
            .messages
            .iter()
            .filter(|m| {
                if let Some(p) = project {
                    if m.project != p {
                        return false;
                    }
                }
                if let Some(s) = session {
                    if m.session_id != s {
                        return false;
                    }
                }
                if let Some(ref ql) = q_lower {
                    if !m.display.to_lowercase().contains(ql.as_str()) {
                        return false;
                    }
                }
                if let Some(f) = from {
                    if m.timestamp < f {
                        return false;
                    }
                }
                if let Some(t) = to {
                    if m.timestamp > t {
                        return false;
                    }
                }
                true
            })
            .collect();

        let total = filtered.len();
        let messages: Vec<Message> = filtered
            .into_iter()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect();

        MessagesResponse { total, messages }
    }

    pub fn projects(&self) -> Vec<ProjectInfo> {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for m in self.messages.iter() {
            *counts.entry(m.project.as_str()).or_insert(0) += 1;
        }
        let mut result: Vec<ProjectInfo> = counts
            .into_iter()
            .map(|(path, count)| ProjectInfo {
                path: path.to_string(),
                count,
            })
            .collect();
        result.sort_by(|a, b| b.count.cmp(&a.count));
        result
    }

    pub fn sessions(&self, project: Option<&str>) -> Vec<SessionInfo> {
        let mut map: HashMap<&str, SessionInfo> = HashMap::new();
        for m in self.messages.iter() {
            if let Some(p) = project {
                if m.project != p {
                    continue;
                }
            }
            let entry = map
                .entry(m.session_id.as_str())
                .or_insert_with(|| SessionInfo {
                    session_id: m.session_id.clone(),
                    project: m.project.clone(),
                    first_ts: m.timestamp,
                    count: 0,
                });
            entry.count += 1;
            if m.timestamp < entry.first_ts {
                entry.first_ts = m.timestamp;
            }
        }
        let mut result: Vec<SessionInfo> = map.into_values().collect();
        result.sort_by(|a, b| b.first_ts.cmp(&a.first_ts));
        result
    }

    pub fn stats(&self) -> Stats {
        if self.messages.is_empty() {
            return Stats {
                total_messages: 0,
                total_projects: 0,
                total_sessions: 0,
                earliest_ts: None,
                latest_ts: None,
                daily_counts: vec![],
            };
        }

        let mut projects = std::collections::HashSet::new();
        let mut sessions = std::collections::HashSet::new();
        let mut daily: BTreeMap<String, usize> = BTreeMap::new();
        let (mut earliest, mut latest) = (i64::MAX, i64::MIN);

        for m in self.messages.iter() {
            projects.insert(m.project.as_str());
            sessions.insert(m.session_id.as_str());
            if m.timestamp < earliest {
                earliest = m.timestamp;
            }
            if m.timestamp > latest {
                latest = m.timestamp;
            }
            let date = Utc
                .timestamp_millis_opt(m.timestamp)
                .single()
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            *daily.entry(date).or_insert(0) += 1;
        }

        Stats {
            total_messages: self.messages.len(),
            total_projects: projects.len(),
            total_sessions: sessions.len(),
            earliest_ts: Some(earliest),
            latest_ts: Some(latest),
            daily_counts: daily
                .into_iter()
                .map(|(date, count)| DailyCount { date, count })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_store(lines: &[&str]) -> MessageStore {
        let mut f = NamedTempFile::new().unwrap();
        for line in lines {
            writeln!(f, "{}", line).unwrap();
        }
        MessageStore::load(f.path()).unwrap()
    }

    fn msg_json(display: &str, ts: i64, project: &str, session: &str) -> String {
        serde_json::json!({
            "display": display,
            "pastedContents": {},
            "timestamp": ts,
            "project": project,
            "sessionId": session,
        })
        .to_string()
    }

    #[test]
    fn test_load_skips_bad_lines() {
        let store = make_store(&[
            &msg_json("hello", 1000, "/p/a", "s1"),
            "not valid json",
            &msg_json("world", 2000, "/p/b", "s2"),
        ]);
        assert_eq!(store.messages.len(), 2);
    }

    #[test]
    fn test_query_all() {
        let store = make_store(&[
            &msg_json("hello", 1000, "/p/a", "s1"),
            &msg_json("world", 2000, "/p/b", "s2"),
        ]);
        let resp = store.query_messages(None, None, None, None, None, 0, 100);
        assert_eq!(resp.total, 2);
        assert_eq!(resp.messages.len(), 2);
    }

    #[test]
    fn test_query_filter_project() {
        let store = make_store(&[
            &msg_json("hello", 1000, "/p/a", "s1"),
            &msg_json("world", 2000, "/p/b", "s2"),
        ]);
        let resp = store.query_messages(Some("/p/a"), None, None, None, None, 0, 100);
        assert_eq!(resp.total, 1);
        assert_eq!(resp.messages[0].project, "/p/a");
    }

    #[test]
    fn test_query_search() {
        let store = make_store(&[
            &msg_json("hello world", 1000, "/p/a", "s1"),
            &msg_json("goodbye", 2000, "/p/a", "s1"),
        ]);
        let resp = store.query_messages(None, None, Some("hello"), None, None, 0, 100);
        assert_eq!(resp.total, 1);
    }

    #[test]
    fn test_query_time_range() {
        let store = make_store(&[
            &msg_json("a", 1000, "/p", "s1"),
            &msg_json("b", 2000, "/p", "s1"),
            &msg_json("c", 3000, "/p", "s1"),
        ]);
        let resp = store.query_messages(None, None, None, Some(1500), Some(2500), 0, 100);
        assert_eq!(resp.total, 1);
        assert_eq!(resp.messages[0].display, "b");
    }

    #[test]
    fn test_query_pagination() {
        let store = make_store(&[
            &msg_json("a", 3000, "/p", "s1"),
            &msg_json("b", 2000, "/p", "s1"),
            &msg_json("c", 1000, "/p", "s1"),
        ]);
        let resp = store.query_messages(None, None, None, None, None, 1, 1);
        assert_eq!(resp.total, 3);
        assert_eq!(resp.messages.len(), 1);
        assert_eq!(resp.messages[0].display, "b");
    }

    #[test]
    fn test_projects() {
        let store = make_store(&[
            &msg_json("a", 1000, "/p/a", "s1"),
            &msg_json("b", 2000, "/p/a", "s1"),
            &msg_json("c", 3000, "/p/b", "s2"),
        ]);
        let projects = store.projects();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].path, "/p/a");
        assert_eq!(projects[0].count, 2);
    }

    #[test]
    fn test_sessions() {
        let store = make_store(&[
            &msg_json("a", 1000, "/p/a", "s1"),
            &msg_json("b", 2000, "/p/a", "s1"),
            &msg_json("c", 3000, "/p/b", "s2"),
        ]);
        let sessions = store.sessions(None);
        assert_eq!(sessions.len(), 2);
        let s2 = sessions.iter().find(|s| s.session_id == "s2").unwrap();
        assert_eq!(s2.count, 1);
    }

    #[test]
    fn test_sessions_filter_project() {
        let store = make_store(&[
            &msg_json("a", 1000, "/p/a", "s1"),
            &msg_json("c", 3000, "/p/b", "s2"),
        ]);
        let sessions = store.sessions(Some("/p/a"));
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "s1");
    }

    #[test]
    fn test_stats() {
        let store = make_store(&[
            &msg_json("a", 1000, "/p/a", "s1"),
            &msg_json("b", 2000, "/p/b", "s2"),
        ]);
        let stats = store.stats();
        assert_eq!(stats.total_messages, 2);
        assert_eq!(stats.total_projects, 2);
        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.earliest_ts, Some(1000));
        assert_eq!(stats.latest_ts, Some(2000));
    }

    #[test]
    fn test_empty_store() {
        let store = make_store(&[]);
        let stats = store.stats();
        assert_eq!(stats.total_messages, 0);
        assert!(stats.earliest_ts.is_none());
    }
}
