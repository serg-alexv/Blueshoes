pub mod dns;
pub mod https;
pub mod icmp;
pub mod route;
pub mod system;
pub mod doctor;
pub mod env;
pub mod trace;


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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_telemetry_event_mutation_flag() {
        let event = TelemetryEvent::new("test", "ok", 10, json!({"msg": "test"}));

        // Assert native struct enforcement
        assert!(!event.mutation_performed);

        // Assert JSON serialization includes the constraint
        let serialized = serde_json::to_string(&event).unwrap();
        assert!(serialized.contains(r#""mutation_performed":false"#));
    }
}
