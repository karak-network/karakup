use clap::Args;

pub mod processor;

#[derive(Args, Debug)]
pub struct Installer {
    /// Version to install, leave blank for latest
    #[arg(short = 'v', long)]
    pub version: Option<String>,
}
