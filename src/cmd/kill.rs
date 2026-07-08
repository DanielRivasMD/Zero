use crate::util::shell::exec_sh;

pub struct KillCmd;

impl KillCmd {
    pub fn run() -> anyhow::Result<()> {
        let cmd = "zellij kill-session \"$(zellij list-sessions | grep '(current)' | sed 's/\\x1b\\[[0-9;]*m//g' | awk '{print $1}')\"";
        exec_sh(cmd).map_err(|e| anyhow::anyhow!("Failed to kill current Zellij session: {}", e))
    }
}
