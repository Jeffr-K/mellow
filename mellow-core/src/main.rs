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
            // 1. Lens를 통해 심층 분석 수행 (Result 객체를 받아옴)
            let analysis_result = guard::analyze_command_safety(&command);

            if analysis_result.is_dangerous {
                // 2. Report 모듈을 사용하여 시각적 보고서 출력
                mellow_report::print_report(&analysis_result);

                // 3. Prompt 모듈을 사용하여 사용자 승인 대기
                if prompt::confirm_execution() {
                    println!("🌿 Mellow: Manual override detected. Executing...");
                    execute_flow(&command).await;
                } else {
                    println!("🌿 Mellow: Safety first! Execution aborted.");
                }
            } else {
                // 4. 안전한 경우 즉시 실행
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
