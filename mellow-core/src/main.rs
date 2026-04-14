use clap::Parser;

use crate::cli::{Cli, Commands};

mod cli;
mod executor;
mod guard;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { command } => {
            if guard::is_dangerous(&command) {
                println!("🌿 Mellow: 이 명령어는 시스템에 영향을 줄 수 있어 실행을 멈췄습니다.");
            } else {
                if let Err(e) = executor::run_command(&command).await {
                    eprintln!("Mellow 실행 에러: {}", e);
                }
            }
        }
        Commands::Config => {
            println!("Mellow v0.1.0 - Ready to protect your vibe.");
        }
    }
}
