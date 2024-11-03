pub mod install;

use color_eyre::eyre;

use super::Installer;

pub async fn process(args: Installer) -> eyre::Result<()> {
    if args.version.is_none() {
        install::install_latest().await
    } else {
        install::install_specific(args.version.unwrap()).await
    }
}
