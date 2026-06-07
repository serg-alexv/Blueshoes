use serde::{Serialize, Deserialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct MtuStatus {
    pub interface: String,
    pub mtu: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MtuTelemetry {
    pub event_type: String,
    pub timestamp: u64,
    pub status: Vec<MtuStatus>,
}

pub fn run(interfaces: &[&str]) -> MtuTelemetry {
    let mut results = Vec::new();

    for &iface in interfaces {
        let output = Command::new("cat")
            .arg(format!("/sys/class/net/{}/mtu", iface))
            .output();

        let mtu = match output {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                s.parse::<u32>().ok()
            }
            _ => None,
        };

        results.push(MtuStatus {
            interface: iface.to_string(),
            mtu,
        });
    }

    MtuTelemetry {
        event_type: "probe_mtu".to_string(),
        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        status: results,
    }
}
