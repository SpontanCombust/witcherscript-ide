use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &str = "./editors/vscode";

pub fn prep_client(watch: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;

    sh.change_dir(EXT_DIR);
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