use crate::probes::TelemetryEvent;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

pub fn append_event(event: &TelemetryEvent) -> io::Result<()> {
    // Write to a local dev file for now
    let path = Path::new("events.jsonl");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let json = serde_json::to_string(event)?;
    writeln!(file, "{}", json)
}

pub fn tail_journal(lines_count: usize) -> io::Result<Vec<String>> {
    let path = Path::new("events.jsonl");
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    // Simple approach: read all lines, then take the last N.
    // For a real production app we'd seek from the end, but this is fine for M1.
    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let mut all_lines = lines?;
    
    if all_lines.len() > lines_count {
        let skip = all_lines.len() - lines_count;
        all_lines.drain(0..skip);
    }
    
    Ok(all_lines)
}
