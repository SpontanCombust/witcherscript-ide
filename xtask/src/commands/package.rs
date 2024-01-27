use anyhow::Context;
use xshell::{Shell, cmd};


const LSP_SRC: &str = "./target/release/witcherscript-lsp"; 
const LSP_DST: &str = "./editors/vscode/server/bin"; 
const EXT_DIR: &str = "./editors/vscode";
const VSIX_NAME: &str = "witcherscript-ide.vsix";

pub fn package() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    println!("Building LSP release...");
    cmd!(sh, "cargo build --package witcherscript-lsp --release").run()?;
    
    let lsp_src = if cfg!(unix) {
        LSP_SRC.to_string()
    } else {
        format!("{LSP_SRC}.exe")
    };

    sh.copy_file(lsp_src, LSP_DST)?;
    println!("Copied LSP into {}", LSP_DST);


    sh.change_dir(EXT_DIR);
    
    if cfg!(unix) {
        cmd!(sh, "npm --version").run().with_context(|| "npm is required")?;
        cmd!(sh, "vsce --version").run().with_context(|| "vsce is required:  npm install -g vsce")?;
    
        cmd!(sh, "npm run package").run()?;
    } else {
        cmd!(sh, "cmd.exe /c npm --version").run().with_context(|| "npm is required")?;
        cmd!(sh, "cmd.exe /c vsce --version").run().with_context(|| "vsce is required:  npm install -g vsce")?;
    
        cmd!(sh, "cmd.exe /c npm run package").run()?;
    }

    let version = env!("CARGO_PKG_VERSION");
    sh.copy_file(VSIX_NAME, format!("witcherscript-ide-{version}.vsix"))?;

    sh.remove_path(VSIX_NAME)?;

    Ok(())
}