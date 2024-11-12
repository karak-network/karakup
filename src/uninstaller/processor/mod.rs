pub mod uninstall;

use color_eyre::eyre;

pub async fn process() -> eyre::Result<()> {
    uninstall::uninstall().await
}
