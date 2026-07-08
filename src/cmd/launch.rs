use anyhow::Context;
use clap::Parser;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Launch a new Zellij session with tabs for a root directory and its git‑controlled subdirectories.
#[derive(Parser)]
pub struct LaunchCmd {
    /// Path whose subdirectories will each become a tab (the root itself also gets a tab).
    pub target: String,
}

impl LaunchCmd {
    pub fn run(self) -> anyhow::Result<()> {
        let root = Path::new(&self.target)
            .canonicalize()
            .with_context(|| format!("Invalid path: {}", self.target))?;

        if !root.is_dir() {
            anyhow::bail!("Not a directory: {}", root.display());
        }

        let session_name = root
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();

        // Collect only subdirectories containing a .git folder
        let mut subdirs = Vec::new();
        for entry in fs::read_dir(&root)
            .with_context(|| format!("Failed to read directory: {}", root.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.join(".git").exists() {
                subdirs.push(path);
            }
        }

        // Build KDL layout string
        let mut kdl = String::new();
        kdl.push_str("layout {\n");

        // Root tab
        write_devel_tab(&mut kdl, &session_name, &root.to_string_lossy());

        // Subdirectory tabs
        for dir in &subdirs {
            let name = dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned();
            write_devel_tab(&mut kdl, &name, &dir.to_string_lossy());
        }

        kdl.push_str("}\n");

        // Write temporary KDL file
        let mut tmp = tempfile::Builder::new()
            .prefix("zero_launch_")
            .suffix(".kdl")
            .tempfile()
            .context("Failed to create temporary KDL file")?;
        tmp.write_all(kdl.as_bytes())
            .context("Failed to write temporary KDL content")?;

        // Launch Zellij with the generated layout
        let status = Command::new("zellij")
            .args([
                "--new-session-with-layout",
                tmp.path().to_str().unwrap(),
                "--session",
                &session_name,
            ])
            .status()
            .context("Failed to launch Zellij session")?;

        drop(tmp); // tempfile auto-deletes

        if !status.success() {
            anyhow::bail!("Zellij exited with error");
        }

        Ok(())
    }
}

/// Append a tab with an editor (Helix) and a canvas pane.
fn write_devel_tab(kdl: &mut String, name: &str, cwd: &str) {
    use std::fmt::Write;
    // Vertical split: left pane = editor, right pane = canvas (shell)
    let _ = writeln!(kdl, "    tab name=\"{}\" cwd=\"{}\" {{", name, cwd);
    let _ = writeln!(kdl, "        pane split_direction=\"vertical\" {{");
    let _ = writeln!(kdl, "            pane command=\"hx\" {{");
    let _ = writeln!(kdl, "            }}");
    let _ = writeln!(kdl, "            pane {{");
    let _ = writeln!(kdl, "            }}");
    let _ = writeln!(kdl, "        }}");
    let _ = writeln!(kdl, "    }}");
}
