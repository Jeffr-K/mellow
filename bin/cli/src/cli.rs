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
    Run { command: String },

    Config,
}
