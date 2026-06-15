use crate::util::shell::exec_sh;
use clap::Parser;
use std::collections::HashMap;

// ─── Geometry and layouts ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Geometry {
    height: String,
    width: String,
    x: String,
    y: String,
}

lazy_static::lazy_static! {
    static ref LAYOUT_PRESETS: HashMap<&'static str, Geometry> = {
        let mut m = HashMap::new();
        m.insert("full",         Geometry { height: "100%".into(), width: "100%".into(), x: "0".into(),  y: "0".into()  });
        m.insert("half-left",    Geometry { height: "50%".into(),  width: "100%".into(), x: "0".into(),  y: "0".into()  });
        m.insert("half-right",   Geometry { height: "50%".into(),  width: "100%".into(), x: "50%".into(), y: "0".into()  });
        m.insert("top-left",     Geometry { height: "50%".into(),  width: "50%".into(),  x: "0".into(),  y: "0".into()  });
        m.insert("bottom-left",  Geometry { height: "50%".into(),  width: "53%".into(),  x: "0".into(),  y: "52%".into() });
        m.insert("top-right",    Geometry { height: "50%".into(),  width: "50%".into(),  x: "50%".into(), y: "0".into()  });
        m.insert("bottom-right", Geometry { height: "50%".into(),  width: "53%".into(),  x: "50%".into(), y: "52%".into() });
        m.insert("default",      Geometry { height: "95%".into(),  width: "100%".into(), x: "10".into(), y: "0".into()  });
        m
    };
}

fn resolve_layout(name: &str, flags: &FloatFlags) -> anyhow::Result<Geometry> {
    let preset = LAYOUT_PRESETS.get(name).cloned().ok_or_else(|| {
        let valid: Vec<_> = LAYOUT_PRESETS.keys().collect();
        anyhow::anyhow!("Unknown layout '{}' (must be one of: {:?})", name, valid)
    })?;
    Ok(preset.override_with(flags))
}

impl Geometry {
    fn override_with(&self, flags: &FloatFlags) -> Self {
        Geometry {
            height: override_val(&self.height, &flags.height, "100%"),
            width: override_val(&self.width, &flags.width, "95%"),
            x: override_val(&self.x, &flags.x, "10"),
            y: override_val(&self.y, &flags.y, "0"),
        }
    }
}

fn override_val(preset: &str, flag: &str, fallback: &str) -> String {
    if !preset.is_empty() && preset != fallback {
        return preset.to_string();
    }
    if flag != fallback {
        return flag.to_string();
    }
    fallback.to_string()
}

// ─── Zellij float builder ──────────────────────────────────────────────────

struct ZellijFloat {
    name: String,
    close_on_exit: bool,
    pinned: bool,
    command: String,
    args: Vec<String>,
    geometry: Option<Geometry>,
}

impl ZellijFloat {
    fn new(name: &str, command: &str) -> Self {
        ZellijFloat {
            name: name.into(),
            close_on_exit: false,
            pinned: false,
            command: command.into(),
            args: vec![],
            geometry: None,
        }
    }

    fn with_close_on_exit(mut self) -> Self {
        self.close_on_exit = true;
        self
    }

    fn with_pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    fn with_geometry(mut self, geom: Geometry) -> Self {
        self.geometry = Some(geom);
        self
    }

    fn build_cmd(&self, flags: &FloatFlags) -> String {
        let geom = match &self.geometry {
            Some(g) => g.clone(),
            None => resolve_layout("default", flags).unwrap(),
        };

        let mut zflags = vec![format!("--name {}", self.name)];
        if self.pinned {
            zflags.push("--pinned true".into());
        }
        if self.close_on_exit {
            zflags.push("--close-on-exit".into());
        }
        zflags.push(format!("--height {}", geom.height));
        zflags.push(format!("--width {}", geom.width));
        zflags.push(format!("--x {}", geom.x));
        zflags.push(format!("--y {}", geom.y));

        let mut cmd = format!(
            "zellij run --floating {} -- {}",
            zflags.join(" "),
            self.command
        );
        if !self.args.is_empty() {
            cmd.push(' ');
            cmd.push_str(&self.args.join(" "));
        }
        cmd
    }
}

// ─── Float command ──────────────────────────────────────────────────────────

/// Open a floating pane in Zellij
#[derive(Parser)]
pub struct FloatCmd {
    /// Pane height (e.g. "100%")
    #[arg(short = 'H', long, default_value = "100%")]
    pub height: String,

    /// Pane width (e.g. "95%")
    #[arg(short = 'W', long, default_value = "95%")]
    pub width: String,

    /// Horizontal offset
    #[arg(short = 'X', long, default_value = "10")]
    pub x: String,

    /// Vertical offset
    #[arg(short = 'Y', long, default_value = "0")]
    pub y: String,

    /// Layout preset
    #[arg(value_parser = LAYOUT_PRESETS.keys().copied().collect::<Vec<_>>())]
    pub layout: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<FloatSubcommands>,
}

impl FloatCmd {
    pub fn run(self) -> anyhow::Result<()> {
        match self.subcommand {
            Some(sub) => sub.run(&self),
            None => {
                let layout = self.layout.as_deref().unwrap_or("default");
                let geom = resolve_layout(layout, &FloatFlags::from(&self))?;
                let cmd = ZellijFloat::new("canvas", "zsh")
                    .with_geometry(geom)
                    .with_close_on_exit()
                    .build_cmd(&FloatFlags::from(&self));
                exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch floating shell: {}", e))
            }
        }
    }
}

// ─── Flag helper struct ────────────────────────────────────────────────────

#[derive(Clone)]
struct FloatFlags {
    height: String,
    width: String,
    x: String,
    y: String,
}

impl From<&FloatCmd> for FloatFlags {
    fn from(cmd: &FloatCmd) -> Self {
        FloatFlags {
            height: cmd.height.clone(),
            width: cmd.width.clone(),
            x: cmd.x.clone(),
            y: cmd.y.clone(),
        }
    }
}

// ─── Float subcommands ─────────────────────────────────────────────────────

/// Float subcommands for launching specific tools
#[derive(Parser, Clone)]
pub struct FloatSubcommands {
    /// Optional file or directory argument
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl FloatSubcommands {
    fn flags_from_parent(parent: &FloatCmd) -> FloatFlags {
        FloatFlags::from(parent)
    }

    pub fn run(&self, parent: &FloatCmd) -> anyhow::Result<()> {
        // This is called when run as `float <subcommand>`
        // Dispatch based on which subcommand was invoked
        // Since clap flattens subcommands, we need another way
        // For now, this is handled in the top-level match
        Ok(())
    }

    pub fn run_bat(self) -> anyhow::Result<()> {
        // This is a top-level shortcut; we need a default FloatFlags
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let mut args = vec!["--paging=always".to_string()];
        if let Some(file) = self.args.first() {
            args.push(file.clone());
        }
        let cmd = ZellijFloat::new("bat", "bat")
            .with_args(args)
            .with_close_on_exit()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch bat: {}", e))
    }

    pub fn run_eza(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let mut args = vec![
            "--header".into(),
            "--long".into(),
            "--icons".into(),
            "--classify".into(),
            "--git".into(),
            "--group".into(),
            "--color=always".into(),
        ];
        if let Some(dir) = self.args.first() {
            args.push(dir.clone());
        }
        let cmd = ZellijFloat::new("eza", "eza")
            .with_args(args)
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch eza: {}", e))
    }

    pub fn run_helix(self) -> anyhow::Result<()> {
        self.run_editor("hx", "helix")
    }

    pub fn run_micro(self) -> anyhow::Result<()> {
        self.run_editor("micro", "micro")
    }

    fn run_editor(self, call: &str, name: &str) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let mut args = vec![];
        if let Some(file) = self.args.first() {
            args.push(file.clone());
        }
        let cmd = ZellijFloat::new(name, call)
            .with_args(args)
            .with_close_on_exit()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch {} editor: {}", name, e))
    }

    pub fn run_lazygit(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let layout = self.args.first().map(|s| s.as_str()).unwrap_or("full");
        let geom = resolve_layout(layout, &flags)?;
        let cmd = ZellijFloat::new("lazygit", "lazygit")
            .with_geometry(geom)
            .with_close_on_exit()
            .with_pinned()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch lazygit: {}", e))
    }

    pub fn run_mdcat(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let file = self
            .args
            .first()
            .ok_or_else(|| anyhow::anyhow!("mdcat command requires a file argument"))?;
        let cmd = ZellijFloat::new("canvas", "mdcat")
            .with_args(vec!["--paginate".into(), file.clone()])
            .with_close_on_exit()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch mdcat: {}", e))
    }

    pub fn run_resize(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let layout = self.args.first().map(|s| s.as_str()).unwrap_or("default");
        let geom = resolve_layout(layout, &flags)?;
        let cmd = format!(
            "zellij action rename-pane float
zellij action change-floating-pane-coordinates --pane-id $ZELLIJ_PANE_ID \
--height {} \
--width {} \
--x {} \
--y {}",
            geom.height, geom.width, geom.x, geom.y
        );
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to resize floating pane: {}", e))
    }

    pub fn run_watch(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let cmd = ZellijFloat::new("watch", "just")
            .with_args(vec!["watch".into()])
            .with_close_on_exit()
            .with_pinned()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch watch command: {}", e))
    }

    pub fn run_yazi(self) -> anyhow::Result<()> {
        let flags = FloatFlags {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
        };
        let cmd = ZellijFloat::new("yazi", "yazi")
            .with_close_on_exit()
            .build_cmd(&flags);
        exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch yazi: {}", e))
    }
}
