use crate::util::shell::exec_sh;

pub struct StackCmd;

impl StackCmd {
    pub fn run() -> anyhow::Result<()> {
        exec_sh("zellij action new-pane --stacked --name stack")
            .map_err(|e| anyhow::anyhow!("Failed to stack pane: {}", e))
    }
}
