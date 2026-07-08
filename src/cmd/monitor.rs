use crate::util::shell::exec_sh;

pub struct MonitorCmd;

impl MonitorCmd {
    pub fn run() -> anyhow::Result<()> {
        let cmd = "zellij action new-tab --layout $HOME/.zero/monitor/monitor.kdl --name \"$( [ \"$PWD\" = \"$HOME\" ] && echo \"~\" || basename \"$PWD\" )\"";
        exec_sh(cmd).map_err(|e| anyhow::anyhow!("Failed to launch monitor tab: {}", e))
    }
}
