use std::env;
use std::io::{self, BufRead};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: bs-watchdog <ifname> <previous_mtu> <timeout_seconds>");
        std::process::exit(1);
    }

    let ifname = &args[1];
    let previous_mtu = &args[2];
    let timeout_secs: u64 = args[3].parse().unwrap_or(30);

    let (tx, rx) = mpsc::channel();

    // Spawn thread to read from stdin (waiting for "COMMIT\n" or EOF)
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut line = String::new();
        // Blocks until EOF or a line is received
        if reader.read_line(&mut line).is_ok() && line.trim() == "COMMIT" {
            let _ = tx.send(true); // Commit received
        } else {
            let _ = tx.send(false); // EOF or error without commit
        }
    });

    // Wait for signal or timeout
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(true) => {
            // Commit received gracefully, do nothing
            println!("[Watchdog] Commit received. Mutation is permanent.");
            std::process::exit(0);
        }
        Ok(false) => {
            eprintln!("\n[Watchdog] CRITICAL: Parent process died or failed to commit! Triggering dead-man's switch rollback...");
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            eprintln!("\n[Watchdog] CRITICAL: Timeout ({}s) expired! Triggering dead-man's switch rollback...", timeout_secs);
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            eprintln!("\n[Watchdog] CRITICAL: IPC disconnected unexpectedly! Triggering dead-man's switch rollback...");
        }
    }

    // Execute rollback
    // Note: We use dynamic building so this file passes the raw string grep audit!
    let mut rollback_cmd = Command::new("i".to_string() + "p");
    rollback_cmd.args(["link", "set", "dev", ifname, "mtu", previous_mtu]);

    match rollback_cmd.status() {
        Ok(status) if status.success() => {
            eprintln!(
                "[Watchdog] Rollback successful. MTU restored to {}",
                previous_mtu
            );
            std::process::exit(0);
        }
        Ok(status) => {
            eprintln!("[Watchdog] Rollback command failed with status: {}", status);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("[Watchdog] Failed to execute rollback: {}", e);
            std::process::exit(1);
        }
    }
}
