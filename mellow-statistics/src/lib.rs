use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MellowStats {
    pub total_scans: u64,
    pub blocked_count: u64,
    pub bypassed_count: u64,
}

impl MellowStats {
    /// 통계 파일 저장 경로 (~/.mellow_stats.json)
    fn get_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".mellow_stats.json");
        path
    }

    /// 파일에서 통계 데이터를 불러옵니다. 파일이 없으면 기본값을 반환합니다.
    pub fn load() -> Self {
        fs::read_to_string(Self::get_path())
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    /// 현재 통계 상태를 파일에 저장합니다.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(Self::get_path(), content)
    }

    /// 위험 요소를 성공적으로 차단했을 때 기록합니다.
    pub fn record_block(&mut self) {
        self.total_scans += 1;
        self.blocked_count += 1;
        let _ = self.save();
    }

    /// 위험 요소를 인지했지만 사용자가 실행을 승인했을 때 기록합니다.
    pub fn record_bypass(&mut self) {
        self.total_scans += 1;
        self.bypassed_count += 1;
        let _ = self.save();
    }

    /// 위험 요소가 없는 깨끗한 실행을 기록합니다.
    pub fn record_clean_scan(&mut self) {
        self.total_scans += 1;
        let _ = self.save();
    }
}
