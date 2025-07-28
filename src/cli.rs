use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "LocalBridge")]
#[command(version = "0.1.0")]
#[command(about = "🌉 Share your localhost securely over the internet", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the tunnel server
    Server {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Connect your local app to a remote tunnel
    Connect {
        #[arg(short, long)]
        local_port: u16,

        #[arg(short, long)]
        remote_port: u16,

        #[arg(short, long)]
        token: Option<String>,

        #[arg(long, default_value_t = 0)]
        ttl: u64, // in seconds
    },
}
