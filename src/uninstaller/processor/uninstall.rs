use std::path::PathBuf;

use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;

use crate::constants::{CLI_NAME, INSTALL_DIR};

pub async fn uninstall() -> eyre::Result<()> {
    let install_dir = PathBuf::from(&*INSTALL_DIR);
    let install_path = install_dir.join(CLI_NAME);

    if install_path.exists() {
        println!("\n{}", "Uninstalling Karak CLI...".purple());
        println!(
            "\n{} {}",
            "Removing Karak CLI -".yellow(),
            install_path.display()
        );
        std::fs::remove_file(install_path)?;
        println!("\n{}", "Karak CLI uninstalled successfully".green().bold());
    } else {
        println!("\n{}", "Karak CLI is not installed".red());
        println!("\n{}", "Run `karakup install` to install it".blue());
    }

    Ok(())
}
