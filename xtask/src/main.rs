use clap::Parser;
use cli::Cli;

mod cli;
mod commands;


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::PrepLsp { release, target } => commands::prep_lsp(release, target),
        cli::Commands::Package { out_dir, out_name } => commands::package(out_dir, out_name),
        cli::Commands::Install => commands::install()
    }
}