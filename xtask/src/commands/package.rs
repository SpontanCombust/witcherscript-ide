use std::path::Path;
use anyhow::Context;
use xshell::{Shell, cmd};


const EXT_DIR: &'static str = "editors/vscode";
const VSIX_NAME: &'static str = "witcherscript-ide.vsix";
const NPM: &'static str = if cfg!(windows) { "npm.cmd" } else { "npm" };
const NPX: &'static str = if cfg!(windows) { "npx.cmd" } else { "npx" };

pub fn package(output: Option<String>, code_target: Option<String>, pre_release: bool) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;
    let cwd = std::env::current_dir()?;

    let ext_dir = root.join(EXT_DIR);
    sh.change_dir(ext_dir);
    
    let output_path = Path::new(output.as_ref().map(|o| o.as_str()).unwrap_or(VSIX_NAME));
    let vsix_dst = if output_path.is_absolute() {
        output_path.to_path_buf()
    } else {
        cwd.join(output_path)
    };
    let mut package_opt_args = Vec::new();
    if let Some(code_target) = &code_target {
        package_opt_args.extend(["--target", code_target])
    }
    if pre_release {
        package_opt_args.extend(["--pre-release"]);
    }

    cmd!(sh, "{NPM} --version").run().with_context(|| "npm is required")?;
    cmd!(sh, "{NPM} ci").run()?;
    cmd!(sh, "{NPX} vsce package -o {vsix_dst} {package_opt_args...}").run()?;

    println!("Packaged the extension into {}", vsix_dst.display());

    Ok(())
}