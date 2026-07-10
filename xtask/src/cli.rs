use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "xtask",
    about = "Shared repository maintenance tasks.",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Update pinned Cargo dependencies from stayhydated/shared master.
    UpdateSharedRevisions {
        /// Git workspace containing the Cargo manifests to update.
        #[arg(long, default_value = ".")]
        workspace_root: PathBuf,
    },
}
