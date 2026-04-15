use ast::AnalysisResult;
use colored::*;
use stat::MellowStats;

pub fn print_report(result: &AnalysisResult, stats: &MellowStats) {
    println!(
        "\n{}",
        "🔍 Mellow Detailed Security Analysis".bright_cyan().bold()
    );
    println!("{}", "━".repeat(50).bright_cyan());

    for finding in &result.findings {
        println!(
            "{} {} at {}:{}",
            "⚠️".yellow(),
            "Risk Detected:".bold().red(),
            "Line".bright_black(),
            finding.line.to_string().bright_yellow()
        );
        println!("   {}", finding.message.italic());
        println!("{}", "━".repeat(50).bright_black());
    }

    println!(
        "📈 누적 보안 통계: 총 스캔 {}회 | 차단 {}회 | 승인 {}회",
        stats.total_scans,
        stats.blocked_count.to_string().green(),
        stats.bypassed_count.to_string().red()
    );
}
