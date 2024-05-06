use std::path::MAIN_SEPARATOR_STR;
use xshell::{Shell, cmd};


const LSP_DST: &str = "editors/vscode/server/bin";

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


    let mut lsp_dir = root.join("target");
    if let Some(target) = &target {
        lsp_dir.push(target);
    }
    
    if release {
        lsp_dir.push("release");
    } else {
        lsp_dir.push("debug");
    }
    

    let mut lsp_bin = lsp_dir.join("witcherscript-lsp");

    if cfg!(windows) {
        lsp_bin.set_extension("exe");
    }

    // make sure destination folder exists
    let lsp_dst = root.join(LSP_DST.replace('/', MAIN_SEPARATOR_STR));
    sh.create_dir(&lsp_dst)?;

    sh.copy_file(lsp_bin, &lsp_dst)?;
    println!("Copied LSP binary into {}", lsp_dst.display());


    if cfg!(windows) && !release {
        let lsp_pdb = lsp_dir.join("witcherscript_lsp.pdb");

        sh.copy_file(lsp_pdb, &lsp_dst)?;
        println!("Copied LSP .pdb into {}", lsp_dst.display());
    }

    Ok(())
}