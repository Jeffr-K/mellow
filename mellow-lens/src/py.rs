use tree_sitter::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

/// 파이썬 소스코드를 분석하여 위험한 시스템 호출(os.system)이 있는지 확인합니다.
pub fn scan_python_safety(code: &str) -> bool {
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
    let matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    matches.count() > 0
}
