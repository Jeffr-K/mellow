pub mod py;
pub mod ts;
use rayon::prelude::*;

pub enum Language {
    Python,
    TypeScript,
}

pub fn analyze(code: &str, lang: Language) -> bool {
    match lang {
        Language::Python => py::scan_python_safety(code),
        Language::TypeScript => ts::scan_ts_safety(code),
    }
}

pub fn analyze_parallel(targets: Vec<(&str, Language)>) -> bool {
    // rayon의 par_iter를 사용하여 모든 파일을 코어별로 분산 분석
    targets.into_par_iter().any(|(code, lang)| match lang {
        Language::Python => py::scan_python_safety(code),
        Language::TypeScript => ts::scan_ts_safety(code),
    })
}
