use crate::{AnalysisResult, Finding};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

pub fn scan_ts_safety(code: &str) -> AnalysisResult {
    let mut parser = Parser::new();

    // TS 파서 로드 (LANGUAGE_TYPESCRIPT 사용)
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    parser
        .set_language(&language)
        .expect("Error loading TS grammar");

    let tree = parser.parse(code, None).expect("Failed to parse TS code");

    // 위험 패턴 쿼리 (eval, child_process 호출 및 임포트 감지)
    let query_str = r#"
        ; 1. 직접 함수 호출 탐지 (eval, exec)
        (call_expression
          function: (identifier) @func_name
          (#match? @func_name "^(eval|exec)$"))

        ; 2. 메소드 형태 호출 탐지 (execSync, spawn 등)
        (call_expression
          function: (member_expression
            property: (property_identifier) @method)
          (#match? @method "^(exec|execSync|spawn)$"))

        ; 3. 위험 모듈 require 탐지
        (call_expression
          function: (identifier) @require
          arguments: (arguments (string (string_fragment) @mod_name))
          (#eq? @require "require")
          (#match? @mod_name "^(child_process|fs|vm)$"))
    "#;

    let query = Query::new(&language, query_str).expect("Failed to create TS query");
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    let mut findings = Vec::new();

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            let start_pos = node.start_position();

            // 캡처된 노드에 따라 메시지 분기 처리 (선택 사항)
            findings.push(Finding {
                line: start_pos.row + 1,
                column: start_pos.column + 1,
                message: "Security-sensitive API call detected (eval/exec/require)".to_string(),
            });
        }
    }

    AnalysisResult {
        is_dangerous: !findings.is_empty(),
        findings,
    }
}
