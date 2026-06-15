use crate::util::shell::exec_sh;
use clap::Parser;
use std::env;

/// Launch a new Zellij session with a custom layout
#[derive(Parser)]
pub struct LaunchCmd {
    /// The .kdl layout file to launch (required)
    #[arg(short, long)]
    pub layout: String,

    /// If set, cd into this path before launching (and return afterward)
    #[arg(short, long)]
    pub target: Option<String>,
}

impl LaunchCmd {
    pub fn run(self) -> anyhow::Result<()> {
        let shell_launch = format!(
            "zellij action write-chars \"zellij --new-session-with-layout $HOME/.zero/launch/{}\"; zellij action write 13",
            self.layout
        );

        if self.target.is_none() {
            return exec_sh(&shell_launch)
                .map_err(|e| anyhow::anyhow!("Failed to launch new Zellij session: {}", e));
        }

        let target = self.target.as_ref().unwrap();
        let cmd_tab = "zellij action new-tab --layout $HOME/.zero/layouts/launch.kdl --name \"$( [ \"$PWD\" = \"$HOME\" ] && echo \"~\" || basename \"$PWD\" )\"";
        let full_cmd = format!("{}; {}", cmd_tab, shell_launch);

        let original_dir = env::current_dir()
            .map_err(|e| anyhow::anyhow!("Failed to recall original directory: {}", e))?;

        // Change to target directory
        env::set_current_dir(target).map_err(|e| {
            anyhow::anyhow!("Failed to change to target directory '{}': {}", target, e)
        })?;

        // Execute and restore directory
        let result = exec_sh(&full_cmd);
        env::set_current_dir(&original_dir).ok();

        result
            .map_err(|e| anyhow::anyhow!("Failed to launch new Zellij session with target: {}", e))
    }
}
