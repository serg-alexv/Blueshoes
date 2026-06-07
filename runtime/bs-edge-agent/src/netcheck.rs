use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct NetcheckResult {
    pub success: bool,
    pub target: String,
    pub latency_ms: Option<u64>,
    pub error: Option<String>,
}

pub fn perform_check(target: &str) -> NetcheckResult {
    let start = std::time::Instant::now();
    
    // Bounded timeout to ensure it doesn't hang the rollback loop later
    let agent = ureq::builder()
        .timeout(Duration::from_secs(3))
        .build();

    match agent.get(target).call() {
        Ok(response) => {
            let latency = start.elapsed().as_millis() as u64;
            let success = response.status() >= 200 && response.status() < 300;
            
            NetcheckResult {
                success,
                target: target.to_string(),
                latency_ms: Some(latency),
                error: if !success { Some(format!("HTTP {}", response.status())) } else { None },
            }
        }
        Err(ureq::Error::Status(code, _)) => {
            NetcheckResult {
                success: false,
                target: target.to_string(),
                latency_ms: Some(start.elapsed().as_millis() as u64),
                error: Some(format!("HTTP {}", code)),
            }
        }
        Err(e) => {
            NetcheckResult {
                success: false,
                target: target.to_string(),
                latency_ms: None,
                error: Some(e.to_string()),
            }
        }
    }
}
