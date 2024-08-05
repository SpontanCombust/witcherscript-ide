use std::path::{Path, MAIN_SEPARATOR_STR};
use xshell::{Shell, cmd};


const LSP_ASSETS: &str = "crates/lsp/assets/include";
const LSP_BIN_DST: &str = "editors/vscode/deps/lsp_server/bin";
const LSP_ASSETS_DST: &str = "editors/vscode/deps/lsp_server/assets";

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
    let lsp_bin_dst = root.join(LSP_BIN_DST.replace('/', MAIN_SEPARATOR_STR));
    sh.create_dir(&lsp_bin_dst)?;

    sh.copy_file(lsp_bin, &lsp_bin_dst)?;
    println!("Copied LSP binary into {}", lsp_bin_dst.display());


    if cfg!(windows) && !release {
        let lsp_pdb = lsp_dir.join("witcherscript_lsp.pdb");

        sh.copy_file(lsp_pdb, &lsp_bin_dst)?;
        println!("Copied LSP .pdb into {}", lsp_bin_dst.display());
    }

    let lsp_assets_src = root.join(LSP_ASSETS.replace('/', MAIN_SEPARATOR_STR));
    let lsp_assets_dst = root.join(LSP_ASSETS_DST.replace('/', MAIN_SEPARATOR_STR));
    sh.remove_path(&lsp_assets_dst)?;
    sh.create_dir(&lsp_assets_dst)?;
    copy_server_assets(&sh, &lsp_assets_src, &lsp_assets_src, &lsp_assets_dst)?;
    println!("Copied LSP assets into {}", lsp_assets_dst.display());

    Ok(())
}

fn copy_server_assets(sh: &Shell, src_dir_root: &Path, src_dir: &Path, dst_root: &Path) -> anyhow::Result<()> {
    for src_entry in std::fs::read_dir(src_dir)? {
        let src_entry = src_entry?;
        let ty = src_entry.file_type()?;
        let src_path = src_entry.path();
        let dst_path = dst_root.join(src_path.strip_prefix(src_dir_root)?);
        if ty.is_dir() {
            sh.create_dir(&dst_path)?;
            copy_server_assets(sh, src_dir_root, &src_path, dst_root)?;
        // READMEs and stuff are not needed
        } else if dst_path.extension().unwrap() != "md" {
            sh.copy_file(src_path, dst_path)?;
        }
    }

    Ok(())
}
