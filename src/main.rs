////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use clap::Parser;

////////////////////////////////////////////////////////////////////////////////////////////////////

mod cli;
mod cmd;
mod util;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Clean => cmd::clean::CleanCmd::run(),
        cli::Command::Kill => cmd::kill::KillCmd::run(),
        cli::Command::Launch(cmd) => cmd.run(),
        cli::Command::List => cmd::list::ListCmd::run(),
        cli::Command::Name => cmd::name::NameCmd::run(),
        cli::Command::Monitor => cmd::monitor::MonitorCmd::run(),
        cli::Command::Stack => cmd::stack::StackCmd::run(),
        cli::Command::Tab(cmd) => cmd.run(),
        cli::Command::Update => cmd::update::UpdateCmd::run(),
        cli::Command::Float(cmd) => cmd.run(),

        cli::Command::Bat(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Eza(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Helix(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Lazygit(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Mdcat(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Micro(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Resize(sub) => {
            let flags = crate::cmd::float::FloatFlags {
                height: "100%".into(),
                width: "95%".into(),
                x: "10".into(),
                y: "0".into(),
            };
            crate::cmd::float::run_resize(&sub, &flags)
        }
        cli::Command::Watch(_sub) => cmd::float::FloatCmd::default().run(),
        cli::Command::Yazi(_sub) => cmd::float::FloatCmd::default().run(),

        cli::Command::Identity => cmd::identity::run(),
        cli::Command::Completion { shell } => cmd::completion::run(shell),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
