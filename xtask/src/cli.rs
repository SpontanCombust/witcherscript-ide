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
    /// Build and copy LSP server executable into VSCode's extension directory
    PrepServer {
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
        watch: bool,
        /// Whether client should be built instantly by skipping `npm ci` step
        #[arg(long)]
        fast: bool
    },
    /// Build and package VSCode extension into a .vsix file
    Package {
        /// Output path for the .vsix file; default is "./witcherscript-ide.vsix"
        #[arg(short, long)]
        out: Option<String>,
        /// VSCode extension target, e.g. win32-x64 
        /// 
        /// https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions
        #[arg(long)]
        target: Option<String>
    },
    /// Build, package and install the VSCode extension
    Install
}
