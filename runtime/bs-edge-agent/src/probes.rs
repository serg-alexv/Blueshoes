use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct SystemStatus {
    pub mem_total_kb: Option<u64>,
    pub mem_free_kb: Option<u64>,
    pub load_avg_1m: Option<f64>,
    pub os_name: String,
}

pub fn get_system_status() -> SystemStatus {
    let (mem_total_kb, mem_free_kb) = read_meminfo();
    let load_avg_1m = read_loadavg();
    let os_name = read_os_release();

    SystemStatus {
        mem_total_kb,
        mem_free_kb,
        load_avg_1m,
        os_name,
    }
}

fn read_meminfo() -> (Option<u64>, Option<u64>) {
    let Ok(contents) = fs::read_to_string("/proc/meminfo") else {
        return (None, None);
    };

    let mut total = None;
    let mut free = None;

    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            total = parse_kb(line);
        } else if line.starts_with("MemFree:") {
            free = parse_kb(line);
        }
    }

    (total, free)
}

fn parse_kb(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].parse::<u64>().ok()
    } else {
        None
    }
}

fn read_loadavg() -> Option<f64> {
    let Ok(contents) = fs::read_to_string("/proc/loadavg") else {
        return None;
    };
    let parts: Vec<&str> = contents.split_whitespace().collect();
    if !parts.is_empty() {
        parts[0].parse::<f64>().ok()
    } else {
        None
    }
}

fn read_os_release() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string();
            }
        }
    }
    "Unknown OS".to_string()
}
