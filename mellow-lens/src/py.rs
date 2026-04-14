use tree_sitter::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

use crate::{AnalysisResult, Finding};

pub fn scan_python_safety(code: &str) -> AnalysisResult {
    let mut parser = Parser::new();

    // tree-sitter-python v0.25.0 언어 로드
    let language = tree_sitter_python::LANGUAGE.into();
    parser
        .set_language(&language)
        .expect("Error loading Python grammar");

    let tree = parser
        .parse(code, None)
        .expect("Failed to parse Python code");

    // os.system(...) 호출을 감지하는 S-expression 쿼리
    let query_str = r#"
        (call
          function: (attribute
            object: (identifier) @obj
            attribute: (identifier) @method)
          (#eq? @obj "os")
          (#eq? @method "system")
        ) @danger
    "#;

    let query = Query::new(&language, query_str).expect("Failed to create Python query");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    let mut findings = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            let start_pos = node.start_position();
            findings.push(Finding {
                line: start_pos.row + 1, // 0-indexed를 1-indexed로 변환
                column: start_pos.column + 1,
                message: "Dangerous system call detected: 'os.system'".to_string(),
            });
        }
    }

    AnalysisResult {
        is_dangerous: !findings.is_empty(),
        findings,
    }
}
