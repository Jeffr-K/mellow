use tokio::process::Command;

pub async fn run_command(command_str: &str) -> Result<(), std::io::Error> {
    let mut parts = command_str.split_whitespace();
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();

        let mut child = Command::new(cmd).args(args).spawn()?;

        child.wait().await?;
    }
    Ok(())
}
