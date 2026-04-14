use tree_sitter::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

pub fn scan_ts_safety(code: &str) -> bool {
    let mut parser = Parser::new();
    // TS는 특이하게 typescript()와 tsx()가 구분되기도 하니 주의하세요.
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    parser
        .set_language(&language)
        .expect("Error loading TS grammar");

    let tree = parser.parse(code, None).expect("Failed to parse TS code");

    // 1. eval() 호출 탐지
    // 2. child_process.exec() 호출 탐지
    let query_str = r#"
        ; 직접 함수 호출 탐지
        (call_expression
          function: (identifier) @func_name
          (#match? @func_name "^(eval|exec)$"))

        ; 메소드 형태 호출 탐지 (예: os.system과 유사한 패턴)
        (call_expression
          function: (member_expression
            property: (property_identifier) @method)
          (#match? @method "^(exec|execSync|spawn)$"))

        ; 모듈 임포트 탐지 (위험한 모듈 로딩)
        (call_expression
          function: (identifier) @require
          arguments: (arguments (string (string_fragment) @mod_name))
          (#eq? @require "require")
          (#match? @mod_name "^(child_process|fs|vm)$"))
    "#;

    let query = Query::new(&language, query_str).expect("Failed to create TS query");
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    matches.count() > 0
}
