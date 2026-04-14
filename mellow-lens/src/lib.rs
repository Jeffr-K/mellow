pub mod py;
pub mod ts;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub enum Language {
    Python,
    TypeScript,
}

#[derive(Debug, Clone)]
pub struct Finding {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub is_dangerous: bool,
    pub findings: Vec<Finding>,
}

pub fn analyze(code: &str, lang: Language) -> AnalysisResult {
    match lang {
        Language::Python => py::scan_python_safety(code),
        Language::TypeScript => ts::scan_ts_safety(code),
    }
}

pub fn analyze_parallel(targets: Vec<(&str, Language)>) -> AnalysisResult {
    // 모든 파일의 분석 결과를 수집합니다.
    let results: Vec<AnalysisResult> = targets
        .into_par_iter()
        .map(|(code, lang)| analyze(code, lang))
        .collect();

    let mut all_findings = Vec::new();
    let mut is_dangerous = false;

    for res in results {
        if res.is_dangerous {
            is_dangerous = true;
            all_findings.extend(res.findings);
        }
    }

    AnalysisResult {
        is_dangerous,
        findings: all_findings,
    }
}
