use clap::Parser;
use cli::Cli;

mod cli;
mod commands;


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::PrepLsp { release, target } => commands::prep_lsp(release, target),
        cli::Commands::Package { out_dir } => commands::package(out_dir),
        cli::Commands::Install => commands::install()
    }
}