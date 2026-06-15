use clap::{CommandFactory, Parser};
use clap_complete::{Shell, generate};
use std::io;

use crate::cli::Cli;

/// Generate shell completions
#[derive(Parser)]
pub struct CompletionCmd {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

impl CompletionCmd {
    pub fn run(self) -> anyhow::Result<()> {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();
        generate(self.shell, &mut cmd, &name, &mut io::stdout());
        Ok(())
    }
}
