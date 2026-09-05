use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "xtask-dummy",
    about = "Dummy project maintenance tasks.",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build generated dummy project artifacts.
    Build {
        #[command(subcommand)]
        target: BuildCommand,
    },
    /// Preview generated dummy project artifacts.
    Preview {
        #[command(subcommand)]
        target: PreviewCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum BuildCommand {
    /// Build the Bevy UI wasm example into the Dioxus public tree.
    BevyDemo,
    /// Build mdBook documentation to dummy/web-dummy/public/book.
    Book,
    /// Build the GPUI Kit wasm example into the Dioxus public tree.
    GpuiDemo,
    /// Build llms.txt from mdBook sources to dummy/web-dummy/public/llms.txt.
    LlmsTxt,
    /// Build the Dioxus site into dummy/web-dummy/dist.
    Web,
}

#[derive(Debug, Subcommand)]
pub enum PreviewCommand {
    /// Preview the generated static site with its GitHub Pages base path.
    Web,
}
