pub mod install;

use color_eyre::eyre;

use super::Installer;

pub async fn process(args: Installer) -> eyre::Result<()> {
    match args.version {
        Some(version) => install::install_specific(version).await,
        None => install::install_latest().await,
    }
}
