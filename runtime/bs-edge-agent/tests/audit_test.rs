use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn test_no_mutating_commands_in_source() {
    let forbidden_strings = vec![
        "uci set",
        "uci commit",
        "nft add",
        "nft delete",
        "iptables -A",
        "iptables -I",
        "ip route add",
        "ip route del",
        "wg set",
    ];

    let mut src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    src_dir.push("src");

    let mut found_violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    let content = fs::read_to_string(entry.path()).expect("Failed to read file");
                    for (line_num, line) in content.lines().enumerate() {
                        for forbidden in &forbidden_strings {
                            if line.contains(forbidden) {
                                // Allow comments that discuss the forbidden strings
                                if !line.trim_start().starts_with("//") {
                                    found_violations.push(format!(
                                        "File: {}, Line {}: contains forbidden string '{}'",
                                        entry.path().display(),
                                        line_num + 1,
                                        forbidden
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !found_violations.is_empty() {
        panic!(
            "M1 Audit Failed! Mutating commands found in source code:\n{}",
            found_violations.join("\n")
        );
    }
}
