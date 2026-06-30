// src/utils/file_system.rs
use dirs::config_dir;
use std::fs;
use std::path::PathBuf;

/// Returns the path to the configuration file.
///
/// It constructs a path in the user's configuration directory, like `~/.config/ama/config.toml`.
/// If the directory does not exist, it will be created.
pub fn config_path() -> PathBuf {
    let mut path = config_location();
    path.push("config.toml");
    path
}

/// Returns the path to the configuration directory.
fn config_location() -> PathBuf {
    let mut path = config_dir().expect("Unable to determine the config directory.");
    path.push("ama");
    fs::create_dir_all(&path).expect("Failed to create config directory");
    path
}
