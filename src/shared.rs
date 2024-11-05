use std::{
    env::{
        consts::{ARCH, OS},
        var,
    },
    fs::{self, File},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};

use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest;

use crate::constants::{CLI_NAME, INSTALL_DIR};

pub async fn install_version(version: Option<String>) -> eyre::Result<()> {
    let (platform, os, vendor) = match OS {
        "linux" => ("unknown", "linux", "gnu"),
        "macos" => ("apple", "darwin", ""),
        _ => return Err(eyre::eyre!("Unsupported operating system: {}", OS)),
    };

    let architecture = match ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => return Err(eyre::eyre!("Unsupported architecture: {}", ARCH)),
    };

    let octocrab = octocrab::instance();
    let repo = octocrab.repos("karak-network", "karak-rs");
    let releases_api = repo.releases();

    // Get either specific version or latest
    let release = match version {
        Some(ref v) => releases_api
            .get_by_tag(&format!("karak-cli-v{}", v))
            .await
            .map_err(|_| eyre::eyre!("Version {} not found", v))?,
        None => releases_api.get_latest().await?,
    };

    let asset_name = if OS == "macos" {
        format!("karak-cli-{architecture}-{platform}-{os}.tar.xz")
    } else {
        format!("karak-cli-{architecture}-{platform}-{os}-{vendor}.tar.xz",)
    };

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .ok_or(eyre::eyre!(
            "No matching release asset found for {asset_name}"
        ))?;

    let version_display = version.unwrap_or(
        release
            .tag_name
            .strip_prefix("karak-cli-v")
            .unwrap_or(&release.tag_name)
            .to_string(),
    );
    println!(
        "\n{}{}\n",
        "📦 Downloading Karak CLI version - ".cyan(),
        version_display.cyan()
    );

    // Create progress bar
    let progress_bar = ProgressBar::new(asset.size as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] {bar:40.white} {bytes}/{total_bytes} ({eta})",
            )?
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
    );

    // Create install directory if it doesn't exist
    let install_dir = PathBuf::from(INSTALL_DIR);
    fs::create_dir_all(&install_dir)?;

    println!("\n\n{}", "📝 Downloading and extracting...".purple());

    // Stream the download directly to tar
    let client = reqwest::Client::new();
    let mut response = client
        .get(asset.browser_download_url.as_str())
        .send()
        .await?;

    let mut child = Command::new("tar")
        .args([
            "xf",
            "-", // Read from stdin
            "--strip-components",
            "1",
            "-C",
            install_dir.to_str().unwrap(),
        ])
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        stdin.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        progress_bar.set_position(downloaded);
    }

    drop(stdin); // Close stdin to signal EOF to tar

    let status = child.wait()?;
    if !status.success() {
        return Err(eyre::eyre!("Failed to extract archive"));
    }

    progress_bar.finish_with_message("Download and extraction completed");

    // Rename the binary to CLI_NAME
    let extracted_binary = install_dir.join("karak");
    let final_binary = install_dir.join(CLI_NAME);
    fs::rename(extracted_binary, &final_binary)?;

    // Set permissions on the binary
    fs::set_permissions(&final_binary, fs::Permissions::from_mode(0o755))?;

    fs::write(install_dir.join(".bin_version"), &version_display)?;

    println!(
        "{} {}",
        "\n✨ Successfully installed Karak CLI to".green(),
        install_dir.display()
    );

    if !path_contains(&install_dir) {
        println!(
            "\n{} {}",
            "⚠️  To complete installation, add the following to your shell configuration:\n"
                .yellow(),
            "\n\texport PATH=\"$HOME/.karak/bin:$PATH\"".cyan()
        );
    }

    Ok(())
}

fn path_contains(dir: &PathBuf) -> bool {
    match var("PATH") {
        Ok(path) => path.split(':').any(|p| PathBuf::from(p) == *dir),
        Err(_) => false,
    }
}

pub async fn get_latest_version() -> eyre::Result<String> {
    let octocrab = octocrab::instance();
    let repo = octocrab.repos("karak-network", "karak-rs");
    let latest_release = repo.releases().get_latest().await?;

    // Remove 'karak-cli-v' prefix from the tag name
    let version = latest_release
        .tag_name
        .strip_prefix("karak-cli-v")
        .unwrap_or(&latest_release.tag_name)
        .to_string();

    Ok(version)
}

pub async fn get_current_version() -> eyre::Result<String> {
    let version_file = PathBuf::from(INSTALL_DIR).join(".bin_version");
    let version = fs::read_to_string(version_file)?;
    Ok(version)
}
