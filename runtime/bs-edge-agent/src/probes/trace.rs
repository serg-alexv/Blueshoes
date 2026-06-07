use serde_json::json;
use std::process::Command;
use crate::probes::TelemetryEvent;
use std::time::Instant;

pub fn run(target: &str) -> TelemetryEvent {
    let start = Instant::now();
    let mut status = "ok";
    let mut trace_output = String::new();

    // Check if traceroute is available
    if let Ok(output) = Command::new("traceroute")
        .arg("-m")
        .arg("15") // Max 15 hops for safe bounded execution
        .arg("-w")
        .arg("1") // 1 sec wait time
        .arg("-q")
        .arg("1") // 1 query per hop
        .arg("-n") // Numeric only (no DNS)
        .arg(target)
        .output()
    {
        if output.status.success() {
            trace_output = String::from_utf8_lossy(&output.stdout).to_string();
        } else {
            status = "error_trace_failed";
            trace_output = String::from_utf8_lossy(&output.stderr).to_string();
        }
    } else {
        status = "error_traceroute_not_found";
    }

    let duration_ms = start.elapsed().as_millis() as u64;

    TelemetryEvent::new("trace", status, duration_ms, json!({
        "target": target,
        "output": trace_output,
    }))
}
