use serde_json::json;
use std::env;
use crate::probes::TelemetryEvent;

pub fn run() -> TelemetryEvent {
    let mut safe_env = serde_json::Map::new();

    let safe_keys = vec!["PATH", "SHELL", "USER", "HOME", "LANG", "TERM", "PWD"];

    for (key, value) in env::vars() {
        if safe_keys.contains(&key.as_str()) {
            safe_env.insert(key, json!(value));
        }
    }

    // Add architecture explicitly
    safe_env.insert("ARCH".to_string(), json!(env::consts::ARCH));
    safe_env.insert("OS".to_string(), json!(env::consts::OS));

    TelemetryEvent::new("env", "ok", 0, json!(safe_env))
}
