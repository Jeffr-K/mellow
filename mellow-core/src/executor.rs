use tokio::process::Command;

pub async fn run_command(command_str: &str) -> Result<(), std::io::Error> {
    // 공백으로 명령어를 분리하여 실행
    let mut parts = command_str.split_whitespace();
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();

        let mut child = Command::new(cmd).args(args).spawn()?;

        child.wait().await?;
    }
    Ok(())
}
