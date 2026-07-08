use crate::util::shell::exec_sh;
use clap::{Parser, Subcommand};
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

    static ref VALID_LAYOUTS: Vec<&'static str> = {
        LAYOUT_PRESETS.keys().copied().collect()
    };
}

fn resolve_layout(name: &str, flags: &FloatFlags) -> anyhow::Result<Geometry> {
    let preset = LAYOUT_PRESETS.get(name).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown layout '{}' (must be one of: {:?})",
            name,
            *VALID_LAYOUTS
        )
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
#[derive(Parser, Clone)]
pub struct FloatCmd {
    /// Pane height (e.g. "100%")
    #[arg(short = 'H', long, default_value = "100%", global = true)]
    pub height: String,

    /// Pane width (e.g. "95%")
    #[arg(short = 'W', long, default_value = "95%", global = true)]
    pub width: String,

    /// Horizontal offset
    #[arg(short = 'X', long, default_value = "10", global = true)]
    pub x: String,

    /// Vertical offset
    #[arg(short = 'Y', long, default_value = "0", global = true)]
    pub y: String,

    /// Layout preset
    #[arg(value_parser = VALID_LAYOUTS.clone())]
    pub layout: Option<String>,

    #[command(subcommand)]
    pub subcommand: Option<FloatSubcommands>,
}

impl FloatCmd {
    pub fn run(self) -> anyhow::Result<()> {
        match self.subcommand {
            Some(ref sub) => sub.run(&self),
            None => {
                let layout = self.layout.as_deref().unwrap_or("default");
                let flags = FloatFlags::from(&self);
                let geom = resolve_layout(layout, &flags)?;
                let cmd = ZellijFloat::new("canvas", "zsh")
                    .with_geometry(geom)
                    .with_close_on_exit()
                    .build_cmd(&flags);
                exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch floating shell: {}", e))
            }
        }
    }

    fn flags(&self) -> FloatFlags {
        FloatFlags::from(self)
    }
}

// ─── Flag helper struct ────────────────────────────────────────────────────

#[derive(Clone)]
pub struct FloatFlags {
    pub height: String,
    pub width: String,
    pub x: String,
    pub y: String,
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

// ─── Float subcommands enum ─────────────────────────────────────────────────

#[derive(Subcommand, Clone)]
pub enum FloatSubcommands {
    /// View file with bat in floating pane
    Bat(FloatSubArgs),
    /// Browse directory with eza in floating pane
    Eza(FloatSubArgs),
    /// Edit with Helix in floating pane
    #[command(alias = "hx")]
    Helix(FloatSubArgs),
    /// Open lazygit in floating pane
    #[command(alias = "lg")]
    Lazygit(FloatSubArgs),
    /// Render Markdown with mdcat in floating pane
    Mdcat(FloatSubArgs),
    /// Edit with micro in floating pane
    #[command(alias = "mc")]
    Micro(FloatSubArgs),
    /// Resize current floating pane
    Resize(FloatSubArgs),
    /// Run 'just watch' in floating pane
    Watch(FloatSubArgs),
    /// Open yazi file manager in floating pane
    Yazi(FloatSubArgs),
}

/// Arguments for float subcommands
#[derive(Parser, Clone)]
pub struct FloatSubArgs {
    /// Optional file or directory argument
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

impl FloatSubcommands {
    fn run(&self, parent: &FloatCmd) -> anyhow::Result<()> {
        let flags = parent.flags();
        match self {
            FloatSubcommands::Bat(sub) => run_bat(sub, &flags),
            FloatSubcommands::Eza(sub) => run_eza(sub, &flags),
            FloatSubcommands::Helix(sub) => run_helix(sub, &flags),
            FloatSubcommands::Lazygit(sub) => run_lazygit(sub, &flags),
            FloatSubcommands::Mdcat(sub) => run_mdcat(sub, &flags),
            FloatSubcommands::Micro(sub) => run_micro(sub, &flags),
            FloatSubcommands::Resize(sub) => run_resize(sub, &flags),
            FloatSubcommands::Watch(sub) => run_watch(sub, &flags),
            FloatSubcommands::Yazi(sub) => run_yazi(sub, &flags),
        }
    }
}

// ─── Run functions ─────────────────────────────────────────────────────────

fn run_bat(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let mut bat_args = vec!["--paging=always".to_string()];
    if let Some(file) = args.args.first() {
        bat_args.push(file.clone());
    }
    let cmd = ZellijFloat::new("bat", "bat")
        .with_args(bat_args)
        .with_close_on_exit()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch bat: {}", e))
}

fn run_eza(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let mut eza_args = vec![
        "--header".into(),
        "--long".into(),
        "--icons".into(),
        "--classify".into(),
        "--git".into(),
        "--group".into(),
        "--color=always".into(),
    ];
    if let Some(dir) = args.args.first() {
        eza_args.push(dir.clone());
    }
    let cmd = ZellijFloat::new("eza", "eza")
        .with_args(eza_args)
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch eza: {}", e))
}

fn run_helix(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    run_editor(args, flags, "hx", "helix")
}

fn run_micro(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    run_editor(args, flags, "micro", "micro")
}

fn run_editor(
    args: &FloatSubArgs,
    flags: &FloatFlags,
    call: &str,
    name: &str,
) -> anyhow::Result<()> {
    let mut editor_args = vec![];
    if let Some(file) = args.args.first() {
        editor_args.push(file.clone());
    }
    let cmd = ZellijFloat::new(name, call)
        .with_args(editor_args)
        .with_close_on_exit()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch {} editor: {}", name, e))
}

fn run_lazygit(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let layout = args.args.first().map(|s| s.as_str()).unwrap_or("full");
    let geom = resolve_layout(layout, flags)?;
    let cmd = ZellijFloat::new("lazygit", "lazygit")
        .with_geometry(geom)
        .with_close_on_exit()
        .with_pinned()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch lazygit: {}", e))
}

fn run_mdcat(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let file = args
        .args
        .first()
        .ok_or_else(|| anyhow::anyhow!("mdcat command requires a file argument"))?;
    let cmd = ZellijFloat::new("canvas", "mdcat")
        .with_args(vec!["--paginate".into(), file.clone()])
        .with_close_on_exit()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch mdcat: {}", e))
}

pub fn run_resize(args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let layout = args.args.first().map(|s| s.as_str()).unwrap_or("default");
    let geom = resolve_layout(layout, flags)?;
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

fn run_watch(_args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let cmd = ZellijFloat::new("watch", "just")
        .with_args(vec!["watch".into()])
        .with_close_on_exit()
        .with_pinned()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch watch command: {}", e))
}

fn run_yazi(_args: &FloatSubArgs, flags: &FloatFlags) -> anyhow::Result<()> {
    let cmd = ZellijFloat::new("yazi", "yazi")
        .with_close_on_exit()
        .build_cmd(flags);
    exec_sh(&cmd).map_err(|e| anyhow::anyhow!("Failed to launch yazi: {}", e))
}
