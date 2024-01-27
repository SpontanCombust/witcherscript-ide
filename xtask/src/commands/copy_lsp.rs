use xshell::{Shell, cmd};


const SRC: &str = "./target/debug/witcherscript-lsp";
const DST: &str = "./editors/vscode/server/bin";

pub fn copy_lsp() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    println!("Building the LSP...");
    cmd!(sh, "cargo build --package witcherscript-lsp").run()?;

    let src = if cfg!(unix) {
        SRC.to_string()
    } else {
        format!("{SRC}.exe")
    };

    sh.copy_file(src, DST)?;
    println!("Copied debug LSP into {}", DST);

    Ok(())
}