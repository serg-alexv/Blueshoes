use super::TelemetryEvent;
use serde_json::json;
use std::time::{Duration, Instant};

pub fn run(target: &str) -> TelemetryEvent {
    let start = Instant::now();
    let mut evidence = json!({});
    evidence["target"] = json!(target);
    
    // Bounded timeout
    let agent = ureq::builder()
        .timeout(Duration::from_secs(3))
        .build();

    match agent.get(target).call() {
        Ok(response) => {
            let status_code = response.status();
            evidence["http_status"] = json!(status_code);
            let success = status_code >= 200 && status_code < 300;
            
            let status = if success { "ok" } else { "warn" };
            if !success { 
                evidence["error"] = json!(format!("HTTP {}", status_code));
            }
            
            TelemetryEvent::new("https", status, start.elapsed().as_millis() as u64, evidence)
        }
        Err(ureq::Error::Status(code, _)) => {
            evidence["http_status"] = json!(code);
            evidence["error"] = json!(format!("HTTP {}", code));
            TelemetryEvent::new("https", "fail", start.elapsed().as_millis() as u64, evidence)
        }
        Err(e) => {
            evidence["error"] = json!(e.to_string());
            TelemetryEvent::new("https", "fail", start.elapsed().as_millis() as u64, evidence)
        }
    }
}
