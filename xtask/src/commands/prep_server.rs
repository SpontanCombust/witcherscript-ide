use xshell::{Shell, cmd};


const LSP_DST: &str = "./editors/vscode/server/bin";

pub fn prep_server(release: bool, target: Option<String>) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;
  
    let mut build = cmd!(sh, "cargo build --package witcherscript-lsp");

    let mut lsp_src = root.join("target");
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
    let lsp_dst = root.join(LSP_DST);
    sh.create_dir(&lsp_dst)?;

    sh.copy_file(lsp_src, &lsp_dst)?;
    println!("Copied LSP into {}", lsp_dst.display());

    Ok(())
}