use xshell::Shell;


const SRC: &str = "./target/release/witcherscript-lsp.exe";
const DST: &str = "./editors/vscode/server/bin";

pub fn copy_lsp_release() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    sh.copy_file(SRC, DST)?;
    println!("Copied release LSP into {}", DST);

    Ok(())
}