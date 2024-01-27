use xshell::{Shell, cmd};


const SRC: &str = "./target/release/witcherscript-lsp";
const DST: &str = "./editors/vscode/server/bin";

pub fn copy_lsp_release() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    println!("Building the LSP...");
    cmd!(sh, "cargo build --package witcherscript-lsp --release").run()?;
    
    let src = if cfg!(unix) {
        SRC.to_string()
    } else {
        format!("{SRC}.exe")
    };

    // make sure DST exists
    sh.create_dir(DST)?;

    sh.copy_file(src, DST)?;
    println!("Copied release LSP into {}", DST);

    Ok(())
}