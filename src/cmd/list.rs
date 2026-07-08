use crate::util::shell::exec_sh;

pub struct ListCmd;

impl ListCmd {
    pub fn run() -> anyhow::Result<()> {
        exec_sh("zellij list-sessions")
            .map_err(|e| anyhow::anyhow!("Failed to list Zellij sessions: {}", e))
    }
}
