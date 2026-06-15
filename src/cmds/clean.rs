use crate::util::shell::exec_sh;

pub struct CleanCmd;

impl CleanCmd {
    pub fn run() -> anyhow::Result<()> {
        exec_sh("zellij delete-all-sessions --yes")
            .map_err(|e| anyhow::anyhow!("Failed to clean Zellij sessions: {}", e))
    }
}
