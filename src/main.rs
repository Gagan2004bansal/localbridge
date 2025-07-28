mod cli;
mod client;
mod server;
mod tunnel;
mod stats;
mod auth;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

#[tokio::main]

async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Server { port } => {
            println!("{}", "🚀 Starting LocalBridge Tunnel Server...".green());
            server::run_server(*port).await;
        }
        Commands::Connect {
            local_port,
            remote_port,
            token,
            ttl,
        } => {
            println!("{}", "🔌 Connecting to tunnel server...".green());
            let remote_host = "localbridge-production.up.railway.app"; // <-- Replace with actual Railway domain
            let _ = client::run_client(*local_port, remote_host, *remote_port).await;
        }        
    }
}
