use crate::probes::TelemetryEvent;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

fn get_journal_path() -> PathBuf {
    let primary = Path::new("/var/lib/blueshoes");

    // Attempt to use primary path if we can create/access it
    if fs::create_dir_all(primary).is_ok() {
        // Double check if we can actually write a test file or just assume we can
        // If we have write permissions to the dir, we use it
        let test_file = primary.join(".write_test");
        if File::create(&test_file).is_ok() {
            let _ = fs::remove_file(test_file);
            return primary.join("events.jsonl");
        }
    }

    // Fallback path
    let fallback = Path::new("./target/blueshoes-dev");
    let _ = fs::create_dir_all(fallback);
    fallback.join("events.jsonl")
}

pub fn append_event(event: &TelemetryEvent) -> io::Result<()> {
    let path = get_journal_path();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let json = serde_json::to_string(event)?;
    writeln!(file, "{}", json)
}

pub fn tail_journal(lines_count: usize) -> io::Result<Vec<String>> {
    let path = get_journal_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let mut all_lines = lines?;

    if all_lines.len() > lines_count {
        let skip = all_lines.len() - lines_count;
        all_lines.drain(0..skip);
    }

    Ok(all_lines)
}
