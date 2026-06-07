use super::TelemetryEvent;
use serde_json::json;
use std::process::Command;
use std::time::Instant;

pub fn run() -> TelemetryEvent {
    let start = Instant::now();
    let mut evidence = json!({});
    
    // Explicit read-only command
    match Command::new("ip").args(["route", "show"]).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            evidence["routing_table"] = json!(stdout);

            let mut default_route = None;
            for line in stdout.lines() {
                if line.starts_with("default") {
                    default_route = Some(line.to_string());
                    break;
                }
            }
            evidence["default_route"] = json!(default_route);

            let status = if output.status.success() { "ok" } else { "warn" };
            if !output.status.success() {
                evidence["stderr"] = json!(String::from_utf8_lossy(&output.stderr).to_string());
            }
            
            TelemetryEvent::new("route", status, start.elapsed().as_millis() as u64, evidence)
        }
        Err(e) => {
            evidence["error"] = json!(e.to_string());
            TelemetryEvent::new("route", "fail", start.elapsed().as_millis() as u64, evidence)
        }
    }
}
