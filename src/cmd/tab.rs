use crate::util::shell::exec_sh;
use clap::Parser;
use std::env;

/// Create a new Zellij tab with a workspace layout
#[derive(Parser)]
pub struct TabCmd {
    /// Workspace type
    #[arg(short, long, default_value = "devel", value_parser = ["devel", "tab", "tabs2", "tabs3", "tabs4", "tabs5", "explore", "repl"])]
    pub r#type: String,

    /// Target directories
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub targets: Vec<String>,
}

impl TabCmd {
    pub fn run(self) -> anyhow::Result<()> {
        let tab_type = &self.r#type;

        if self.targets.is_empty() {
            return create_tab(tab_type, None);
        }

        let orig_tab_id = env::var("ZELLIJ_PANE_ID").ok();

        for target in &self.targets {
            create_tab(tab_type, Some(target))?;
        }

        if let Some(tab_id) = orig_tab_id {
            let switch_back = format!("zellij action go-to-tab {}", tab_id);
            exec_sh(&switch_back).ok(); // Non-fatal
        }

        Ok(())
    }
}

pub fn create_tab(tab_type: &str, target: Option<&str>) -> anyhow::Result<()> {
    let cmd = format!(
        "zellij action new-tab --layout $HOME/.zero/tab/{}.kdl --name \"$( [ \"$PWD\" = \"$HOME\" ] && echo \"~\" || basename \"$PWD\" )\"",
        tab_type
    );

    match target {
        None => exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch tab: {}", e)),
        Some(target_dir) => {
            let original_dir = env::current_dir()
                .map_err(|e| anyhow::anyhow!("Failed to recall working directory: {}", e))?;

            env::set_current_dir(target_dir).map_err(|e| {
                anyhow::anyhow!(
                    "Failed to change to target directory '{}': {}",
                    target_dir,
                    e
                )
            })?;

            let result = exec_sh(&cmd);

            env::set_current_dir(&original_dir).ok();

            result.map_err(|e| anyhow::anyhow!("Failed to launch tab: {}", e))
        }
    }
}
