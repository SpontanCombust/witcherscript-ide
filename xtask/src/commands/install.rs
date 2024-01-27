use anyhow::{Context, bail};
use xshell::{Shell, cmd};


const EXT_DIR: &str = "./editors/vscode";
const VSIX_NAME: &str = "witcherscript-ide.vsix";

pub fn install() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    sh.change_dir(EXT_DIR);

    if cfg!(unix) {
        cmd!(sh, "npm --version").run().with_context(|| "npm is required")?;
        cmd!(sh, "code --version").run().with_context(|| "Visual Studio Code is required")?;
    
        cmd!(sh, "npm ci").run()?;
        cmd!(sh, "npm run package").run()?;

    } else {
        cmd!(sh, "cmd.exe /c npm --version").run().with_context(|| "npm is required")?;
        cmd!(sh, "cmd.exe /c code --version").run().with_context(|| "Visual Studio Code is required")?;
    
        cmd!(sh, "cmd.exe /c npm ci").run()?;
        cmd!(sh, "cmd.exe /c npm run package").run()?;
    }

    let installed_extensions = if cfg!(unix) {
        cmd!(sh, "code --install-extension {VSIX_NAME} --force").run()?;
        cmd!(sh, "code --list-extensions").read()?
    } else {
        cmd!(sh, "cmd.exe /c code --install-extension {VSIX_NAME} --force").run()?;
        cmd!(sh, "cmd.exe /c code --list-extensions").read()?
    };

    if !installed_extensions.contains("witcherscript-ide") {
        bail!("Could not install the Visual Studio Code extension.");
    }

    // Remove the vsix file
    // If you want to keep it use xtask package instead
    sh.remove_path(VSIX_NAME)?;
    
    Ok(())
}