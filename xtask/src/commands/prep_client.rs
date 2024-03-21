use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &str = "./editors/vscode";

pub fn prep_client(watch: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;

    let ext_dir = root.join(EXT_DIR).canonicalize()?;
    sh.change_dir(ext_dir);

    let command = if watch { "watch" } else { "build" };

    if cfg!(unix) {
        cmd!(sh, "npm --version").run().with_context(|| "npm is required")?;
    
        cmd!(sh, "npm run {command}").run()?;
    } else {
        cmd!(sh, "cmd.exe /c npm --version").run().with_context(|| "npm is required")?;
    
        cmd!(sh, "cmd.exe /c npm run {command}").run()?;
    }

    Ok(())
}