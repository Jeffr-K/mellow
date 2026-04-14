use crate::cli::{Cli, Commands};
use clap::Parser;

mod cli;
mod executor;
mod guard;
mod prompt;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { command } => {
            if guard::is_dangerous(&command) {
                if prompt::confirm_execution() {
                    println!("🌿 Mellow: Manual override detected. Executing...");
                    execute_flow(&command).await;
                } else {
                    println!("🌿 Mellow: Safety first! Execution aborted.");
                }
            } else {
                execute_flow(&command).await;
            }
        }
        Commands::Config => {
            println!("Mellow v0.1.0 - Your vibe is protected.");
        }
    }
}
async fn execute_flow(command: &str) {
    if let Err(e) = executor::run_command(command).await {
        eprintln!("Mellow Runtime Error: {}", e);
    }
}
