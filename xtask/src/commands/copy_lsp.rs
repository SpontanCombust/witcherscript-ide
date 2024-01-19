use xshell::{Shell, cmd};


const SRC: &str = "./target/debug/witcherscript-lsp.exe";
const DST: &str = "./editors/vscode/server/bin";

pub fn copy_lsp() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    println!("Building the LSP...");
    cmd!(sh, "cargo build --package witcherscript-lsp").run()?;

    sh.copy_file(SRC, DST)?;
    println!("Copied debug LSP into {}", DST);

    Ok(())
}