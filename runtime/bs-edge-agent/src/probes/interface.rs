use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceStatus {
    pub interface: String,
    pub state: String,
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceTelemetry {
    pub event_type: String,
    pub timestamp: u64,
    pub status: Vec<InterfaceStatus>,
}

pub fn run(interfaces: &[&str]) -> InterfaceTelemetry {
    let mut results = Vec::new();

    for &iface in interfaces {
        let output = Command::new("cat")
            .arg(format!("/sys/class/net/{}/operstate", iface))
            .output();

        let (exists, state) = match output {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                (true, s)
            }
            _ => (false, "unknown".to_string()),
        };

        results.push(InterfaceStatus {
            interface: iface.to_string(),
            state,
            exists,
        });
    }

    InterfaceTelemetry {
        event_type: "probe_interface".to_string(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        status: results,
    }
}
