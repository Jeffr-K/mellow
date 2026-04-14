use mellow_lens::{Language, analyze};
use rayon::prelude::*;
use std::fs;

pub fn is_dangerous(command: &str) -> bool {
    let black_list = vec!["sudo ", "rm -rf", "chmod "];
    if black_list.iter().any(|&danger| command.contains(danger)) {
        return true;
    }

    let mut files_to_scan = Vec::new();
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

    if !files_to_scan.is_empty() {
        return check_files_parallel(files_to_scan);
    }

    false
}

fn check_files_parallel(files: Vec<(String, Language)>) -> bool {
    files
        .into_par_iter()
        .any(|(content, lang)| analyze(&content, lang))
}
