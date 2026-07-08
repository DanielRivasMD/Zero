use crate::util::shell::exec_sh;

pub struct UpdateCmd;

impl UpdateCmd {
    pub fn run() -> anyhow::Result<()> {
        exec_sh("zellij action new-tab --layout ~/.zero/launch/update.kdl")
            .map_err(|e| anyhow::anyhow!("Failed to run update layout: {}", e))
    }
}
