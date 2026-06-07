pub mod system;
pub mod route;
pub mod dns;
pub mod icmp;
pub mod https;

use serde::Serialize;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Clone)]
pub struct TelemetryEvent {
    pub timestamp_utc: u64,
    pub event_type: String,
    pub probe: String,
    pub status: String,
    pub duration_ms: u64,
    pub evidence: Value,
    pub mutation_performed: bool, // Guaranteed to be false
}

impl TelemetryEvent {
    pub fn new(probe: &str, status: &str, duration_ms: u64, evidence: Value) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            timestamp_utc: now,
            event_type: "telemetry_probe".to_string(),
            probe: probe.to_string(),
            status: status.to_string(),
            duration_ms,
            evidence,
            mutation_performed: false, // Strict compliance with M1 spec
        }
    }
}
