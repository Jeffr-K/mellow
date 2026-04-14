use std::fs;

use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

pub fn is_dangerous(command: &str) -> bool {
    let black_list = vec!["rm ", "sudo ", "chmod ", "curl ", "wget "];

    if black_list.iter().any(|&danger| command.contains(danger)) {
        return true;
    }

    if let Some(file_path) = command.split_whitespace().find(|s| s.ends_with(".py")) {
        if let Ok(content) = fs::read_to_string(file_path) {
            return scan_python_safety(&content);
        }
    }

    false
}

pub fn scan_python_safety(code: &str) -> bool {
    let mut parser = Parser::new();
    let language = tree_sitter_python::LANGUAGE.into();
    parser
        .set_language(&language)
        .expect("cannot load python language express.");
    let tree = parser.parse(code, None).expect("Failed to code analysis.");
    let query_str = r#"
            (call
              function: (attribute
                object: (identifier) @obj
                attribute: (identifier) @method)
              (#eq? @obj "os")
              (#eq? @method "system")
            ) @danger
        "#;

    let query = Query::new(&language, query_str).unwrap();
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    matches.count() > 0
}
