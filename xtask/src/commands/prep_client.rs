use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &'static str = "editors/vscode";
const NPM: &'static str = if cfg!(windows) { "npm.cmd" } else { "npm" };

pub fn prep_client(watch: bool, fast: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;

    let ext_dir = root.join(EXT_DIR);
    sh.change_dir(ext_dir);

    let command = if watch { "watch" } else { "build" };
    
    if !fast {
        cmd!(sh, "{NPM} --version").run().with_context(|| "npm is required")?;
        cmd!(sh, "{NPM} ci").run()?;
    }

    cmd!(sh, "{NPM} run {command}").run()?;

    Ok(())
}