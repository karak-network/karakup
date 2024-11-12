use std::env;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static::lazy_static! {
    pub static ref INSTALL_DIR: String = format!("{}/{}", env::var("HOME").unwrap_or_default(), ".karak/bin");
    pub static ref CONFIG_DIR: String = format!("{}/{}", env::var("HOME").unwrap_or_default(), ".karak");
}

pub const CLI_NAME: &str = "karak";
