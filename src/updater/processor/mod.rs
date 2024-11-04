pub mod update;

use color_eyre::eyre;

use super::Updater;

pub async fn process(args: Updater) -> eyre::Result<()> {
    match args.version {
        Some(version) => update::update_specific(version).await,
        None => update::update_latest().await,
    }
}
