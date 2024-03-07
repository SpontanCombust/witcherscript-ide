use std::path::PathBuf;
use xshell::{Shell, cmd};


const LSP_DST: &str = "./editors/vscode/server/bin";

pub fn prep_server(release: bool, target: Option<String>) -> anyhow::Result<()> {
    let sh = Shell::new()?;
  
    let mut build = cmd!(sh, "cargo build --package witcherscript-lsp");

    let mut lsp_src = PathBuf::from("./target");
    if let Some(target) = target {
        build = build.arg("--target").arg(&target);
        lsp_src.push(target);
    }

    if release {
        build = build.arg("--release");
        lsp_src.push("release");
    } else {
        lsp_src.push("debug");
    }

    lsp_src.push("witcherscript-lsp");

    if cfg!(windows) {
        lsp_src.set_extension("exe");
    }

    println!("Building the LSP...");
    build.run()?;


    // make sure destination folder exists
    sh.create_dir(LSP_DST)?;

    sh.copy_file(lsp_src, LSP_DST)?;
    println!("Copied LSP into {}", LSP_DST);

    Ok(())
}