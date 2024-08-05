use anyhow::{Context, bail};
use xshell::{Shell, cmd};


const EXT_DIR: &'static str = "editors/vscode";
const VSIX_NAME: &'static str = "witcherscript-ide.vsix";
const NPM: &'static str = if cfg!(windows) { "npm.cmd" } else { "npm" };
const NPX: &'static str = if cfg!(windows) { "npx.cmd" } else { "npx" };
const CODE: &'static str = if cfg!(windows) { "code.cmd" } else { "code" };

pub fn install() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;

    let ext_dir = root.join(EXT_DIR);
    sh.change_dir(ext_dir);

    cmd!(sh, "{NPM} --version").run().with_context(|| "npm is required")?;
    cmd!(sh, "{CODE} --version").run().with_context(|| "Visual Studio Code is required")?;

    cmd!(sh, "{NPM} ci").run()?;
    cmd!(sh, "{NPM} run check").run()?;
    cmd!(sh, "{NPX} vsce package -o {VSIX_NAME}").run()?;

    cmd!(sh, "{CODE} --install-extension {VSIX_NAME} --force").run()?;
    let installed_extensions = cmd!(sh, "{CODE} --list-extensions").read()?;

    if !installed_extensions.contains("witcherscript-ide") {
        bail!("Could not install the Visual Studio Code extension.");
    }

    // Remove the vsix file
    // If you want to keep it use xtask package instead
    sh.remove_path(VSIX_NAME)?;
    
    Ok(())
}