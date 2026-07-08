////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::cmd::{
    float::{FloatCmd, FloatSubArgs},
    launch::LaunchCmd,
    tab::TabCmd,
};
use clap::{Parser, Subcommand, ValueEnum};

////////////////////////////////////////////////////////////////////////////////////////////////////

const HELP: &str = r#""#;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    before_help = concat!(env!("CARGO_PKG_AUTHORS"), "\n", env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION")),
    long_about = HELP,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Enable verbose diagnostics
    #[arg(global = true, short, long)]
    pub verbose: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Subcommand)]
pub enum Command {
    /// Delete all Zellij sessions
    Clean,

    /// Kill the current Zellij session
    Kill,

    /// Launch a new Zellij session with a custom layout
    Launch(LaunchCmd),

    /// List Zellij sessions
    #[command(alias = "ls")]
    List,

    /// Rename current Zellij tab to working directory
    Name,

    /// Monitor system through Zellij tab
    Monitor,

    /// Stack a new pane in current tab
    Stack,

    /// Create a new Zellij tab with layout
    Tab(TabCmd),

    /// Run Zellij update layout
    Update,

    /// Open a floating pane in Zellij
    Float(FloatCmd),

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

    /// Print identity
    #[command(hide = true, aliases = &["id"])]
    Identity,

    /// Generate shell completions
    #[command(hide = true)]
    Completion {
        /// Shell for which to generate completions
        #[arg(value_enum)]
        shell: Shell,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Default for FloatCmd {
    fn default() -> Self {
        FloatCmd {
            height: "100%".into(),
            width: "95%".into(),
            x: "10".into(),
            y: "0".into(),
            layout: None,
            subcommand: None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
