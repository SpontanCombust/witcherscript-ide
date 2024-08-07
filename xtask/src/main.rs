use clap::Parser;
use cli::Cli;

mod cli;
mod commands;


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::PrepServer { release, target } => commands::prep_server(release, target),
        cli::Commands::PrepRw3d => commands::prep_rw3d(),
        cli::Commands::PrepClient { watch, fast } => commands::prep_client(watch, fast),
        cli::Commands::Package { out, target, pre_release } => commands::package(out, target, pre_release),
        cli::Commands::Install => commands::install()
    }
}