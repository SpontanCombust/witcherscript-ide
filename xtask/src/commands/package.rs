use std::path::PathBuf;
use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &str = "./editors/vscode";
const VSIX_NAME: &str = "witcherscript-ide.vsix";

pub fn package(out_dir: Option<String>, out_name: Option<String>) -> anyhow::Result<()> {
    let sh = Shell::new()?;

    // normalize the output path so it stays valid when we change cwd
    let out_dir = if let Some(out_dir) = out_dir {
        // can't just use Option::map because of error propagation here
        Some(PathBuf::from(&out_dir).canonicalize()?)
    } else {
        None
    };

    sh.change_dir(EXT_DIR);
    
    if cfg!(unix) {
        cmd!(sh, "npm --version").run().with_context(|| "npm is required")?;
    
        cmd!(sh, "npm ci").run()?;
        cmd!(sh, "npm run package").run()?;
    } else {
        cmd!(sh, "cmd.exe /c npm --version").run().with_context(|| "npm is required")?;
    
        cmd!(sh, "cmd.exe /c npm ci").run()?;
        cmd!(sh, "cmd.exe /c npm run package").run()?;
    }

    let vsix_file = format!("{}.vsix", out_name.unwrap_or("witcherscript-ide".to_string()));
    let vsix_dst = if let Some(output_dir) = out_dir {
        output_dir.join(vsix_file)
    } else {
        PathBuf::from(&vsix_file).canonicalize()?
    };

    sh.copy_file(VSIX_NAME, vsix_dst.as_os_str())?;
    println!("Copied vsix package into {}", vsix_dst.display());

    // remove the original vsix file
    sh.remove_path(VSIX_NAME)?;

    Ok(())
}