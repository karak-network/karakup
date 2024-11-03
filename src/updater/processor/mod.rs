pub mod update;

use color_eyre::eyre;

use super::Updater;

pub async fn process(args: Updater) -> eyre::Result<()> {
    if args.version.is_none() {
        update::update_latest().await
    } else {
        update::update_specific(args.version.unwrap()).await
    }
}
