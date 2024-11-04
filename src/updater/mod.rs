use clap::Args;

pub mod processor;

#[derive(Args, Debug)]
pub struct Updater {
    /// Version to update to, leave blank for latest
    #[arg(short = 'v', long)]
    pub version: Option<String>,
}
