use std::path::MAIN_SEPARATOR_STR;
use anyhow::Context;
use xshell::{cmd, Shell};


const RW3D_VERSION_TAG: &str = "v0.7.0";
const RW3D_BIN_DST: &str = "editors/vscode/deps/rw3d/bin";


fn rw3d_release_url() -> String {
    let mut url = format!("https://github.com/SpontanCombust/rusty_witcher_debugger/releases/download/{RW3D_VERSION_TAG}/rw3d_cli-{RW3D_VERSION_TAG}-");

    if cfg!(target_os = "windows") {
        url += "x86_64-pc-windows-msvc";
    } else {
        url += "x86_64-unknown-linux-gnu";
    }

    url += ".zip";
    url
}

pub fn prep_rw3d() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    let root = project_root::get_project_root()?;

    let rw3d_url = rw3d_release_url();
    let rw3d_zip_path = root.join("rw3d_cli.zip");
    let rw3d_zip_path_str = rw3d_zip_path.display().to_string();

    println!("Downloading rw3d_cli release zip file...");
    cmd!(sh, "curl -L -o {rw3d_zip_path_str} {rw3d_url}").run()?;

    let rw3d_zip_file = std::fs::File::options()
        .read(true)
        .open(&rw3d_zip_path)
        .context("Failed to read zip archive file")?;
    
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
