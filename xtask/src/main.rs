mod cli;
mod commands;

use clap::Parser as _;

use cli::{Cli, Command};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::UpdateSharedRevisions { workspace_root } => {
            commands::update_shared_revisions::run(&workspace_root)
        },
    }
}
