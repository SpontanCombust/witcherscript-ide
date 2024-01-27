use clap::Parser;
use cli::Cli;

mod cli;
mod commands;


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::CopyLsp => commands::copy_lsp(),
        cli::Commands::CopyLspRelease => commands::copy_lsp_release(),
        cli::Commands::Package { output_dir } => commands::package(output_dir),
        cli::Commands::Install => commands::install()
    }
}