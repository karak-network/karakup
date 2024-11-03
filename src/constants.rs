pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: fix this redundant path
pub const INSTALL_DIR: &str = concat!(env!("HOME"), "/.karak/bin");

pub const CONFIG_DIR: &str = concat!(env!("HOME"), "/.karak");

pub const CLI_NAME: &str = "karak-cli";
