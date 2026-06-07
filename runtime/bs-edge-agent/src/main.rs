mod cli;
mod journal;
mod probes;

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
    }
}
