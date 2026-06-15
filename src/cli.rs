use crate::cmds::{
    clean::CleanCmd,
    completion::CompletionCmd,
    float::{FloatCmd, FloatSubcommands},
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
    /// Generate shell completions
    Completion(CompletionCmd),

    /// Display identity
    #[command(alias = "id", hide = true)]
    Identity,

    /// Delete all Zellij sessions
    Clean,

    /// Kill the current Zellij session
    Kill,

    /// Launch a new Zellij session with a custom layout
    Launch(LaunchCmd),

    /// List Zellij sessions
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
    Bat(FloatSubcommands),

    /// Browse directory with eza in floating pane
    Eza(FloatSubcommands),

    /// Edit with Helix in floating pane
    #[command(alias = "hx")]
    Helix(FloatSubcommands),

    /// Open lazygit in floating pane
    #[command(alias = "lg")]
    Lazygit(FloatSubcommands),

    /// Render Markdown with mdcat in floating pane
    Mdcat(FloatSubcommands),

    /// Edit with micro in floating pane
    #[command(alias = "mc")]
    Micro(FloatSubcommands),

    /// Resize current floating pane
    Resize(FloatSubcommands),

    /// Run 'just watch' in floating pane
    Watch(FloatSubcommands),

    /// Open yazi file manager in floating pane
    Yazi(FloatSubcommands),
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
            Commands::Bat(sub) => sub.run_bat(),
            Commands::Eza(sub) => sub.run_eza(),
            Commands::Helix(sub) => sub.run_helix(),
            Commands::Lazygit(sub) => sub.run_lazygit(),
            Commands::Mdcat(sub) => sub.run_mdcat(),
            Commands::Micro(sub) => sub.run_micro(),
            Commands::Resize(sub) => sub.run_resize(),
            Commands::Watch(sub) => sub.run_watch(),
            Commands::Yazi(sub) => sub.run_yazi(),
        }
    }
}
