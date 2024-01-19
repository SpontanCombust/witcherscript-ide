use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name="xtask")]
#[command(about="Repository automation scripts", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Copy debug build of the LSP server to the VSCode client
    CopyLsp,
    /// Copy release build of the LSP server to the VSCode client
    CopyLspRelease,
    /// Build and package VSCode extension into a .vsix file
    Package,
    /// Build, package and install the VSCode extension
    Install
}
