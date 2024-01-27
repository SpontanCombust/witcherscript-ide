use std::path::PathBuf;
use anyhow::Context;
use xshell::{Shell, cmd};


const LSP_SRC: &str = "./target/release/witcherscript-lsp"; 
const LSP_DST: &str = "./editors/vscode/server/bin"; 
const EXT_DIR: &str = "./editors/vscode";
const VSIX_NAME: &str = "witcherscript-ide.vsix";

pub fn package(output_dir: Option<String>) -> anyhow::Result<()> {
    let sh = Shell::new()?;

    // normalize the output path so it stays valid when we change cwd
    let output_dir = if let Some(output_dir) = output_dir {
        Some(PathBuf::from(&output_dir).canonicalize()?)
    } else {
        None
    };

    println!("Building LSP release...");
    cmd!(sh, "cargo build --package witcherscript-lsp --release").run()?;
    
    let lsp_src = if cfg!(unix) {
        LSP_SRC.to_string()
    } else {
        format!("{LSP_SRC}.exe")
    };

    // make sure DST exists
    sh.create_dir(LSP_DST)?;

    sh.copy_file(lsp_src, LSP_DST)?;
    println!("Copied LSP into {}", LSP_DST);


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

    let version = env!("CARGO_PKG_VERSION");
    let vsix_file = format!("witcherscript-ide-{version}.vsix");
    let vsix_dst = if let Some(output_dir) = output_dir {
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