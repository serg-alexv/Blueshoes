mod cli;
mod probes;
mod netcheck;
mod journal;

use clap::Parser;
use cli::{Cli, Commands};
use serde_json::json;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status { json: _json_flag } => {
            let status = probes::get_system_status();
            println!("{}", serde_json::to_string_pretty(&status).unwrap());
        }
        Commands::Netcheck { json: _json_flag, target } => {
            let result = netcheck::perform_check(target);
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        Commands::Profiles { json: _json_flag } => {
            // Static list for M0
            let profiles = json!([
                {"name": "DIRECT", "description": "Standard OpenWrt routing"},
                {"name": "DNS_PRIVACY", "description": "Encrypted DNS upstreams"},
                {"name": "ECH_PRESERVE", "description": "Preserve TLS integrity"},
                {"name": "USER_TUNNEL", "description": "Operator configured tunnel"}
            ]);
            println!("{}", serde_json::to_string_pretty(&profiles).unwrap());
        }
        Commands::Journal { json: _json_flag } => {
            let entries = journal::read_dummy_journal();
            println!("{}", serde_json::to_string_pretty(&entries).unwrap());
        }
    }
}
