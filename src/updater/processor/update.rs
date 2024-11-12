use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::constants::CONFIG_DIR;
use crate::shared::{get_current_version, get_latest_version, install_version};

fn compare_versions(v1: &str, v2: &str) -> std::cmp::Ordering {
    let v1_parts: Vec<u32> = v1
        .trim()
        .split('.')
        .map(|s| s.parse().unwrap_or_default())
        .collect();
    let v2_parts: Vec<u32> = v2
        .trim()
        .split('.')
        .map(|s| s.parse().unwrap_or_default())
        .collect();

    v1_parts.cmp(&v2_parts)
}

fn get_version_parts(version: &str) -> Vec<u32> {
    version
        .trim()
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

pub async fn update_latest() -> eyre::Result<()> {
    let latest_version = get_latest_version().await?;
    let current_version = get_current_version().await?;

    if compare_versions(&current_version, &latest_version) == std::cmp::Ordering::Equal {
        println!("Latest version {} is already installed", latest_version);
        return Ok(());
    }

    let current_parts = get_version_parts(&current_version);
    let latest_parts = get_version_parts(&latest_version);

    if current_parts[1] < latest_parts[1] {
        println!(
            "{} {}",
            "⚠️  Warning: This is a major update and might break your config file. If you face an error then delete the config at ".yellow(),
            (*CONFIG_DIR).red()
        );
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to proceed with the update?")
            .default(false)
            .interact()?;
        if !confirm {
            println!("Aborting update");
            return Ok(());
        }
    }

    install_version(None).await?;
    Ok(())
}

pub async fn update_specific(version: String) -> eyre::Result<()> {
    let current_version = get_current_version().await?;

    if compare_versions(&current_version, &version) == std::cmp::Ordering::Equal {
        println!("Version {} is already installed", version);
        return Ok(());
    }

    let current_parts = get_version_parts(&current_version);
    let target_parts = get_version_parts(&version);

    if current_parts[1] != target_parts[1] {
        println!(
            "{} {}",
            "⚠️  Warning: This is a major update and might break your config file. If you face an error then delete the config at ".yellow(),
            (*CONFIG_DIR).red()
        );
    }

    install_version(Some(version)).await?;
    Ok(())
}
