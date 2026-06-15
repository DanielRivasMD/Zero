use std::process::Command;

/// Execute a shell command via `sh -c` and return anyhow::Error on failure.
pub fn exec_sh(script: &str) -> anyhow::Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(script)
        .status()
        .map_err(|e| anyhow::anyhow!("Failed to execute shell command: {}", e))?;

    if !status.success() {
        anyhow::bail!(
            "Shell command failed with exit code: {}",
            status.code().unwrap_or(-1)
        );
    }
    Ok(())
}
