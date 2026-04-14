use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod executor;
mod guard;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { command } => {
            if guard::is_dangerous(&command) {
                // Professional and soft English warning
                println!(
                    "🌿 Mellow: Execution blocked. This command may compromise system security."
                );
            } else {
                if let Err(e) = executor::run_command(&command).await {
                    eprintln!("Mellow Runtime Error: {}", e);
                }
            }
        }
        Commands::Config => {
            println!("Mellow v0.1.0 - Your vibe is protected.");
        }
    }
}
