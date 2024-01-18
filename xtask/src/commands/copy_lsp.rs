use xshell::Shell;


const SRC: &str = "./target/debug/witcherscript-lsp.exe";
const DST: &str = "./editors/vscode/server/bin";

pub fn copy_lsp() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    sh.copy_file(SRC, DST)?;
    println!("Copied debug LSP into {}", DST);

    Ok(())
}