use crate::util::shell::exec_sh;

pub struct NameCmd;

impl NameCmd {
    pub fn run() -> anyhow::Result<()> {
        let cmd = "zellij action rename-tab \"$( [ \"$PWD\" = \"$HOME\" ] && echo \"~\" || basename \"$PWD\" )\"";
        exec_sh(cmd).map_err(|e| anyhow::anyhow!("Failed to rename Zellij tab: {}", e))
    }
}
