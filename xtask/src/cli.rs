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
    /// Build and copy LSP server into VSCode's extension directory
    PrepLsp {
        /// Should LSP be built with optimised release profile
        #[arg(long)]
        release: bool,
        /// Compilation target triple
        #[arg(long)]
        target: Option<String>
    },
    /// Build VSCode client
    PrepClient {
        /// Whether client should be continuously watched for changes made to it and rebuilt 
        #[arg(long)]
        watch: bool
    },
    /// Build and package VSCode extension into a .vsix file
    Package {
        /// Output directory for the .vsix file; default is the current working directory
        #[arg(long)]
        out_dir: Option<String>,
        /// Name of the output file without the extension
        #[arg(long)]
        out_name: Option<String>
    },
    /// Build, package and install the VSCode extension
    Install
}
