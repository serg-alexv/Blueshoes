mod cli;
mod journal;
mod probes;
mod profiles;
mod executor;

use clap::Parser;
use cli::{Cli, Commands};
use serde_json::json;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status { json: _json_flag } => {
            let event = probes::system::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Netcheck { json: _json_flag } => {
            // Run all probes sequentially
            let sys_event = probes::system::run();
            let route_event = probes::route::run();
            let dns_event = probes::dns::run("example.com");
            let icmp_event = probes::icmp::run("1.1.1.1");
            let https_event = probes::https::run("https://example.com");

            let events = vec![sys_event, route_event, dns_event, icmp_event, https_event];

            for event in &events {
                if let Err(e) = journal::jsonl::append_event(event) {
                    eprintln!("Failed to write to journal: {}", e);
                }
            }

            println!("{}", serde_json::to_string_pretty(&events).unwrap());
        }
        Commands::Profiles { json: _json_flag } => {
            let profiles = json!([
                {"name": "DIRECT", "description": "Standard OpenWrt routing"},
                {"name": "DNS_PRIVACY", "description": "Encrypted DNS upstreams"},
                {"name": "ECH_PRESERVE", "description": "Preserve TLS integrity"},
                {"name": "USER_TUNNEL", "description": "Operator configured tunnel"}
            ]);
            println!("{}", serde_json::to_string_pretty(&profiles).unwrap());
        }
        Commands::Journal { tail } => {
            let count = tail.unwrap_or(10);
            match journal::jsonl::tail_journal(count) {
                Ok(lines) => {
                    for line in lines {
                        println!("{}", line);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading journal: {}", e);
                }
            }
        }
        Commands::Doctor { json: _json_flag } => {
            let event = probes::doctor::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Env { json: _json_flag } => {
            let event = probes::env::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Canary { json: _json_flag } => {
            use journal::transaction::{TransactionEvent, TransactionState};
            use executor::{Executor, DryRunExecutor};
            
            // Choose executor based on feature flag
            #[cfg(feature = "dangerous_execution")]
            let exec = executor::openwrt::OpenWrtExecutor;
            #[cfg(not(feature = "dangerous_execution"))]
            let exec = DryRunExecutor;

            let tx_id = format!("tx_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

            let profile = profiles::schema::ProfileSchema {
                name: "Canary Test MTU".to_string(),
                intent: profiles::schema::ProfileIntent::SafeMtu,
                description: "Test safe MTU setting".to_string(),
                routes: None,
                dns: None,
            };

            let plan = match journal::planner::Planner::plan(&profile) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Planner failed: {}", e);
                    return;
                }
            };

            let force_dry_run = std::env::var("BS_FORCE_DRY_RUN").unwrap_or_else(|_| "0".to_string()) == "1";
            
            #[cfg(feature = "dangerous_execution")]
            let has_dangerous_execution = true;
            #[cfg(not(feature = "dangerous_execution"))]
            let has_dangerous_execution = false;

            let refusal_reason = if force_dry_run {
                Some("BS_FORCE_DRY_RUN=1 environment variable set")
            } else if !has_dangerous_execution {
                Some("dangerous_execution feature disabled")
            } else if !cli.unsafe_execute {
                Some("missing --unsafe-execute flag")
            } else if cli.confirm.as_deref().unwrap_or("").starts_with("unsafe:") == false {
                Some("missing or invalid --confirm unsafe:<request_id>")
            } else {
                None
            };

            if let Some(reason) = refusal_reason {
                let evidence = journal::planner::Planner::dry_run(&profile, reason).unwrap();
                println!("{}", serde_json::to_string_pretty(&evidence).unwrap());
                
                // Log dry run event
                let dry_run_strings = vec![serde_json::to_string(&evidence).unwrap()];
                let start_event = TransactionEvent::new(tx_id, TransactionState::Start, Some(format!("{:?}", profile.intent)), Some(dry_run_strings));
                let _ = journal::jsonl::append_transaction(&start_event);
                return;
            }

            // --- EXECUTION PATH ---

            let start_event = TransactionEvent::new(tx_id.clone(), TransactionState::Start, Some(format!("{:?}", profile.intent)), None);
            let _ = journal::jsonl::append_transaction(&start_event);
            println!("{}", serde_json::to_string_pretty(&start_event).unwrap());

            // 1. Snapshot
            println!("Capturing snapshot...");
            let snapshot = match exec.capture_snapshot() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to capture snapshot: {}", e);
                    return;
                }
            };

            // 2. Setup dead man's switch timer
            println!("Spawning bs-watchdog...");
            
            #[cfg(feature = "dangerous_execution")]
            let mut watchdog_child = std::process::Command::new(std::env::current_exe().unwrap().parent().unwrap().join("bs-watchdog"))
                .args([&snapshot.metadata, &snapshot.raw_state, "30"])
                .stdin(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to spawn bs-watchdog");

            println!("Applying plan...");
            if let Err(e) = exec.apply(&plan) {
                eprintln!("Failed to apply plan: {}", e);
                let _ = exec.rollback(&snapshot);
                return;
            }

            // Simulate "netcheck" validation process
            println!("Validating network...");
            std::thread::sleep(std::time::Duration::from_secs(5));

            println!("Validation complete. Committing to watchdog.");
            
            #[cfg(feature = "dangerous_execution")]
            {
                use std::io::Write;
                if let Some(mut stdin) = watchdog_child.stdin.take() {
                    let _ = stdin.write_all(b"COMMIT\n");
                }
                let _ = watchdog_child.wait();
            }

            println!("Transaction successful.");
        }
        Commands::Dns { target, json: _json_flag } => {
            let event = probes::dns::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Latency { target, json: _json_flag } => {
            let event = probes::icmp::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Trace { target, json: _json_flag } => {
            let event = probes::trace::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Simulate { json: _json_flag } => {
            use journal::transaction::{TransactionEvent, TransactionState};
            let tx_id = format!("tx_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
            
            // Log intent
            let start_event = TransactionEvent::new(tx_id.clone(), TransactionState::Start, Some("DNS_PRIVACY".to_string()), None);
            if let Err(e) = journal::jsonl::append_transaction(&start_event) {
                eprintln!("Failed to write start to journal: {}", e);
            }
            println!("{}", serde_json::to_string_pretty(&start_event).unwrap());

            // Simulate mutation and success
            std::thread::sleep(std::time::Duration::from_millis(50));
            
            // Log commit
            let commit_event = TransactionEvent::new(tx_id, TransactionState::Commit, None, None);
            if let Err(e) = journal::jsonl::append_transaction(&commit_event) {
                eprintln!("Failed to write commit to journal: {}", e);
            }
            println!("{}", serde_json::to_string_pretty(&commit_event).unwrap());
        }
        Commands::Dummy { json: _json_flag } => {
            println!("Dummy command executed safely.");
        }
    }
}
