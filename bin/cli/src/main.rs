use crate::cli::{Cli, Commands};
use clap::Parser;
use stat::MellowStats;

mod cli;
mod executor;
mod guard;
mod prompt;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut stats = MellowStats::load();

    match &cli.command {
        Commands::Run { command } => {
            let analysis_result = guard::analyze_command_safety(&command);

            if analysis_result.is_dangerous {
                report::print_report(&analysis_result, &stats);

                if prompt::confirm_execution() {
                    // 2. 위험을 인지하고 실행한 경우 기록
                    stats.record_bypass();
                    println!("🌿 Mellow: Manual override detected. Executing...");
                    execute_flow(&command).await;
                } else {
                    // 3. 위험을 감지하여 차단한 경우 기록
                    stats.record_block();
                    println!("🌿 Mellow: Safety first! Execution aborted.");
                }
            } else {
                // 4. 깨끗한 실행 기록
                stats.record_clean_scan();
                execute_flow(&command).await;
            }
        }
        Commands::Config => {
            // 나중에 여기에 현재 통계 요약을 보여주는 기능을 넣으면 좋겠네요!
            println!("Mellow v0.1.0 - Your vibe is protected.");
            println!(
                "Current Stats: Scans({}), Blocked({}), Bypassed({})",
                stats.total_scans, stats.blocked_count, stats.bypassed_count
            );
        }
    }
}

async fn execute_flow(command: &str) {
    if let Err(e) = executor::run_command(command).await {
        eprintln!("Mellow Runtime Error: {}", e);
    }
}
