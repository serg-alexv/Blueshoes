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

    if fs::metadata("/root").is_ok() {
        is_root = true;
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
