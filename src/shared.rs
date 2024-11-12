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

    // Create and download to temp directory
    let temp_dir = tempfile::tempdir()?;
    let download_path = temp_dir.path().join(&asset_name);
    let mut file = File::create(&download_path)?;

    // Stream the download with progress
    let client = reqwest::Client::new();
    let mut response = client
        .get(asset.browser_download_url.as_str())
        .send()
        .await?;

    let mut downloaded: u64 = 0;
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        progress_bar.set_position(downloaded);
    }

    progress_bar.finish_with_message("Download completed");

    println!("\n\n{}", "📝 Extracting archive...".purple());

    // Extract the archive
    let output = Command::new("tar")
        .args([
            "xf",
            download_path.to_str().unwrap(),
            "--strip-components",
            "1",
        ])
        .current_dir(temp_dir.path())
        .output()?;

    if !output.status.success() {
        return Err(eyre::eyre!(
            "Failed to extract archive: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Install the binary
    let install_dir = PathBuf::from(INSTALL_DIR);
    fs::create_dir_all(&install_dir)?;
    let binary_path = temp_dir.path().join("karak");
    let install_path = install_dir.join(CLI_NAME);

    // Remove existing binary if it exists
    if install_path.exists() {
        fs::remove_file(&install_path)
            .map_err(|e| eyre::eyre!("Failed to remove existing binary: {}", e))?;
    }

    // Copy the binary instead of renaming
    fs::copy(&binary_path, &install_path)
        .map_err(|e| eyre::eyre!("Failed to copy binary to install location: {}", e))?;

    // Set permissions
    fs::set_permissions(&install_path, fs::Permissions::from_mode(0o755))?;

    // Clean up the temporary binary
    fs::remove_file(&binary_path).ok(); // ignore error if it fails

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
