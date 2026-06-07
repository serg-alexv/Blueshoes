use crate::probes::TelemetryEvent;
use serde_json::json;
use std::fs;

pub fn run() -> TelemetryEvent {
    let mut status = "ok";
    let mut openwrt_readable = false;
    let mut dev_urandom_readable = false;
    let mut is_root = false;

    // Check OpenWrt release file
    if fs::metadata("/etc/openwrt_release").is_ok() {
        openwrt_readable = true;
    }

    // Check /dev/urandom
    if fs::metadata("/dev/urandom").is_ok() {
        dev_urandom_readable = true;
    }

    // Check if running as root by examining /proc/self/status for Uid line
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("Uid:") {
                if let Some(uid_str) = line.split_whitespace().nth(1) {
                    is_root = uid_str == "0";
                }
                break;
            }
        }
    }

    if !openwrt_readable {
        status = "warning_not_openwrt";
    }

    TelemetryEvent::new(
        "doctor",
        status,
        0,
        json!({
            "openwrt_readable": openwrt_readable,
            "dev_urandom_readable": dev_urandom_readable,
            "is_root": is_root,
        }),
    )
}
