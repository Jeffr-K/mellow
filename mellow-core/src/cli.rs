use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Mellow")]
#[command(about = "Vibe-coding Safe Interceptor", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 명령어를 검증하고 실행합니다.
    Run {
        /// 실행할 명령어 (예: "python app.py")
        command: String,
    },
    /// 설정 및 룰셋을 확인합니다.
    Config,
}
