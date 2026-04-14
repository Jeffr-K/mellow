use colored::*;
use mellow_lens::AnalysisResult;

pub fn print_report(result: &AnalysisResult) {
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

        // 여기에 실제 해당 줄의 코드를 출력하는 로직을 추가하면 좋습니다.
        println!("{}", "━".repeat(50).bright_black());
    }

    if result.is_dangerous {
        println!(
            "\n{} {}",
            "STATUS:".bold(),
            "EXECUTION HALTED".on_red().white().bold()
        );
    }
}
