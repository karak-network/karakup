use std::path::PathBuf;

use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;

use crate::constants::INSTALL_DIR;
use crate::shared::get_current_version;

pub async fn uninstall() -> eyre::Result<()> {
    let install_dir = PathBuf::from(&*INSTALL_DIR);

    if install_dir.exists() {
        println!(
            "\n{} {}",
            "Uninstalling Karak CLI version:".purple(),
            get_current_version().await?.purple()
        );
        println!(
            "\n{} {}",
            "Removing install directory -".yellow(),
            install_dir.display()
        );
        std::fs::remove_dir_all(install_dir)?;
        println!("\n{}", "Karak CLI uninstalled successfully".green().bold());
    } else {
        println!("\n{}", "Karak CLI is not installed".red());
        println!("\n{}", "Run `karakup install` to install it".blue());
    }

    Ok(())
}
