use color_eyre::eyre;

use super::{Command, Root};
use crate::installer;
use crate::uninstaller;
use crate::updater;
pub async fn process(root: Root) -> eyre::Result<()> {
    match root.command {
        Some(Command::Install(args)) => installer::processor::process(args).await,
        Some(Command::Update(args)) => updater::processor::process(args).await,
        Some(Command::Uninstall) => uninstaller::processor::process().await,
        None => Ok(()),
    }
}
