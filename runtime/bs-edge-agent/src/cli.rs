use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check the status of the router (memory, load, OS)
    Status {
        #[arg(long, default_value_t = true)]
        json: bool,
    },
    /// Perform a read-only network validation check
    Netcheck {
        #[arg(long, default_value_t = true)]
        json: bool,
        #[arg(short, long, default_value = "http://1.1.1.1")]
        target: String,
    },
    /// List available static routing profiles
    Profiles {
        #[arg(long, default_value_t = true)]
        json: bool,
    },
    /// Output the local event journal
    Journal {
        #[arg(long, default_value_t = true)]
        json: bool,
    },
}
