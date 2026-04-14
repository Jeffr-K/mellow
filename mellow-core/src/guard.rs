use mellow_lens::analyze_parallel;
use mellow_lens::{AnalysisResult, Language};
use std::fs;

pub fn analyze_command_safety(command: &str) -> AnalysisResult {
    let mut files_to_scan = Vec::new();

    // 1. 명령어 자체 블랙리스트 체크 (기본 Result 생성)
    if command.contains("sudo ") || command.contains("rm -rf") {
        // 명령어 자체가 위험할 경우 즉시 반환 로직 구성 가능
    }

    // 2. 파일 수집
    for word in command.split_whitespace() {
        if word.ends_with(".py") {
            if let Ok(content) = fs::read_to_string(word) {
                files_to_scan.push((content, Language::Python));
            }
        } else if word.ends_with(".ts") || word.ends_with(".js") {
            if let Ok(content) = fs::read_to_string(word) {
                files_to_scan.push((content, Language::TypeScript));
            }
        }
    }

    // 3. 병렬 분석 실행 후 결과 반환
    let targets: Vec<(&str, Language)> = files_to_scan
        .iter()
        .map(|(c, l)| (c.as_str(), l.clone()))
        .collect();

    analyze_parallel(targets)
}

// pub fn is_dangerous(command: &str) -> bool {
//     let black_list = vec!["sudo ", "rm -rf", "chmod "];
//     if black_list.iter().any(|&danger| command.contains(danger)) {
//         return true;
//     }

//     let mut files_to_scan = Vec::new();
//     for word in command.split_whitespace() {
//         if word.ends_with(".py") {
//             if let Ok(content) = fs::read_to_string(word) {
//                 files_to_scan.push((content, Language::Python));
//             }
//         } else if word.ends_with(".ts") || word.ends_with(".js") {
//             if let Ok(content) = fs::read_to_string(word) {
//                 files_to_scan.push((content, Language::TypeScript));
//             }
//         }
//     }

//     let result = check_files_parallel(files_to_scan);
//     result.is_dangerous // AnalysisResult에서 bool 값만 추출
// }

// fn check_files_parallel(files: Vec<(String, Language)>) -> AnalysisResult {
//     let targets: Vec<(&str, Language)> =
//         files.iter().map(|(c, l)| (c.as_str(), l.clone())).collect();
//     mellow_lens::analyze_parallel(targets)
// }
