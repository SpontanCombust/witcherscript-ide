use std::path::MAIN_SEPARATOR_STR;
use anyhow::Context;
use xshell::Shell;


#[cfg(target_os = "windows")]
const RW3D_RELEASE_URL: &str = "https://github.com/SpontanCombust/rusty_witcher_debugger/releases/download/v0.6.1/rw3d_cli-v0.6.1-x86_64-pc-windows-msvc.zip";
#[cfg(target_os = "linux")]
const RW3D_RELEASE_URL: &str = "https://github.com/SpontanCombust/rusty_witcher_debugger/releases/download/v0.6.1/rw3d_cli-v0.6.1-x86_64-unknown-linux-gnu.zip";

const RW3D_BIN_DST: &str = "editors/vscode/deps/rw3d/bin";

pub fn prep_rw3d() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;

    let rw3d_zip_path = root.join("rw3d_cli.zip");
    let mut rw3d_zip_file = std::fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(&rw3d_zip_path)
        .context("Failed to create zip archive file")?;

    println!("Downloading rw3d_cli release zip file...");
    let mut resp = reqwest::blocking::get(RW3D_RELEASE_URL).context("Failed to download rw3d release archive")?;
    std::io::copy(&mut resp, &mut rw3d_zip_file).context("Failed to write downloaded artifact into a file")?;
    
    println!("Unzipping rw3d_cli into target destination...");
    let rw3d_bin_dst = root.join(RW3D_BIN_DST.replace("/", MAIN_SEPARATOR_STR));
    sh.create_dir(&rw3d_bin_dst)?;

    let mut arch = zip::ZipArchive::new(&rw3d_zip_file).context("Failed to read the zip archive")?;
    arch.extract(&rw3d_bin_dst).context("Failed to extract archive contents")?;

    println!("Removing downloaded zip file...");
    sh.remove_path(&rw3d_zip_path)?;

    println!("Successfully downloaded rw3d_cli into {}", rw3d_bin_dst.display());

    Ok(())
}
