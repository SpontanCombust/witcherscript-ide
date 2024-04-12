use xshell::{Shell, cmd};


const LSP_DST: &str = "./editors/vscode/server/bin";

pub fn prep_server(release: bool, target: Option<String>) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;
  
    let mut build_opt_args = Vec::new();
    if let Some(target) = &target {
        build_opt_args.extend(["--target", target]);
    }
    if release {
        build_opt_args.push("--release");
    }

    println!("Building the LSP...");
    cmd!(sh, "cargo build --package witcherscript-lsp {build_opt_args...}").run()?;


    let mut lsp_src = root.join("target");
    if let Some(target) = &target {
        lsp_src.push(target);
    }
    
    if release {
        lsp_src.push("release");
    } else {
        lsp_src.push("debug");
    }
    
    lsp_src.push("witcherscript-lsp");
    
    if cfg!(windows) {
        lsp_src.set_extension("exe");
    }

    // make sure destination folder exists
    let lsp_dst = root.join(LSP_DST);
    sh.create_dir(&lsp_dst)?;

    sh.copy_file(lsp_src, &lsp_dst)?;
    println!("Copied LSP into {}", lsp_dst.display());

    Ok(())
}