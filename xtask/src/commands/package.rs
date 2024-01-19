use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &str = "./editors/vscode";
const VSIX_NAME: &str = "witcherscript-ide.vsix";

pub fn package() -> anyhow::Result<()> {
    let sh = Shell::new()?;

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