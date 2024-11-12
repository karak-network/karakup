use color_eyre::eyre;
use color_eyre::owo_colors::OwoColorize;

use crate::constants::CLI_NAME;
use crate::shared::install_version;

pub async fn install_latest() -> eyre::Result<()> {
    if let Err(e) = install_version(None, false).await {
        println!("{}", e.red());
        return Ok(());
    }

    print_post_install_message();
    Ok(())
}

pub async fn install_specific(version: String) -> eyre::Result<()> {
    if let Err(e) = install_version(Some(version), false).await {
        println!("{}", e.red());
        return Ok(());
    }

    print_post_install_message();
    Ok(())
}

fn print_post_install_message() {
    println!(
        "\nRun `{} {}` to get started",
        CLI_NAME.green().bold(),
        "config update".green().bold()
    );
}
