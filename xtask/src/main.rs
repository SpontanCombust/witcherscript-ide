use clap::Parser;
use cli::Cli;

mod cli;
mod commands;


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::PrepServer { release, target } => commands::prep_server(release, target),
        cli::Commands::PrepClient { watch, fast } => commands::prep_client(watch, fast),
        cli::Commands::Package { out, target } => commands::package(out, target),
        cli::Commands::Install => commands::install()
    }
}