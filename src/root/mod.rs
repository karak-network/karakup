pub mod processor;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

use crate::constants::*;
use crate::installer::Installer;
use crate::updater::Updater;

#[derive(Parser, Debug)]
#[command(version = VERSION, about, long_about = None, propagate_version = true, subcommand_required = false)]
pub struct Root {
    #[arg(long = "completions", value_enum)]
    pub generator: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Install the cli
    #[command(disable_version_flag = true)]
    Install(Installer),

    /// Uninstall the cli
    #[command()]
    Uninstall,

    /// Update the cli to latest or specific version
    #[command(disable_version_flag = true)]
    Update(Updater),
}
