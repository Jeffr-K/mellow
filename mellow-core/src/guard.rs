pub fn is_dangerous(command: &str) -> bool {
    let black_list = vec!["rm ", "sudo ", "chmod ", "curl ", "wget "];

    // 단순 키워드 체크 (Phase 2에서 Tree-sitter로 고도화 예정)
    black_list.iter().any(|&danger| command.contains(danger))
}
