use crate::cmds::{
    clean::CleanCmd,
    completion::CompletionCmd,
    float::{FloatCmd, FloatSubArgs, FloatSubcommands},
    identity::IdentityCmd,
    kill::KillCmd,
    launch::LaunchCmd,
    list::ListCmd,
    monitor::MonitorCmd,
    name::NameCmd,
    stack::StackCmd,
    tab::TabCmd,
    update::UpdateCmd,
};
use clap::{Parser, Subcommand};

/// Zero Zellij overhead
#[derive(Parser)]
#[command(name = "x", version, author, about, long_about = None)]
pub struct Cli {
    /// Enable verbose diagnostics
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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

    /// Display identity
    #[command(alias = "id", hide = true)]
    Identity,

    /// Generate shell completions
    #[command(hide = true)]
    Completion(CompletionCmd),
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Commands::Completion(cmd) => cmd.run(),
            Commands::Identity => IdentityCmd::run(),
            Commands::Clean => CleanCmd::run(),
            Commands::Kill => KillCmd::run(),
            Commands::Launch(cmd) => cmd.run(),
            Commands::List => ListCmd::run(),
            Commands::Name => NameCmd::run(),
            Commands::Monitor => MonitorCmd::run(),
            Commands::Stack => StackCmd::run(),
            Commands::Tab(cmd) => cmd.run(),
            Commands::Update => UpdateCmd::run(),
            Commands::Float(cmd) => cmd.run(),
            Commands::Bat(_sub) => {
                // Top-level shortcuts use default flags
                let cmd = FloatCmd::default();
                cmd.run()
            }
            Commands::Eza(_sub) => FloatCmd::default().run(),
            Commands::Helix(_sub) => FloatCmd::default().run(),
            Commands::Lazygit(_sub) => FloatCmd::default().run(),
            Commands::Mdcat(_sub) => FloatCmd::default().run(),
            Commands::Micro(_sub) => FloatCmd::default().run(),
            Commands::Resize(sub) => {
                let flags = crate::cmds::float::FloatFlags {
                    height: "100%".into(),
                    width: "95%".into(),
                    x: "10".into(),
                    y: "0".into(),
                };
                crate::cmds::float::run_resize(&sub, &flags)
            }
            Commands::Watch(_sub) => FloatCmd::default().run(),
            Commands::Yazi(_sub) => FloatCmd::default().run(),
        }
    }
}

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
