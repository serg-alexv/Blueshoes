use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct JournalEntry {
    pub timestamp: u64,
    pub event_type: String,
    pub detail: String,
}

pub fn read_dummy_journal() -> Vec<JournalEntry> {
    // For M0, we just return a static dummy journal to define the output format
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    vec![
        JournalEntry {
            timestamp: now - 3600,
            event_type: "BOOT".to_string(),
            detail: "bs-edge-agent started in read-only mode".to_string(),
        },
        JournalEntry {
            timestamp: now - 60,
            event_type: "NETCHECK_FAIL".to_string(),
            detail: "Simulated failure for format testing".to_string(),
        },
    ]
}
