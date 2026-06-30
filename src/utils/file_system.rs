use dirs::{config_dir, data_dir};
use std::path::PathBuf;
use std::{env, fs};

pub fn config_location() -> PathBuf {
    env_default_path("AMA_CONFIG_DIR", config_dir)
}

pub fn config_path() -> PathBuf {
    let mut path = config_location();
    path.push("config.toml");
    path
}

pub fn database_path() -> PathBuf {
    let mut path = env_default_path("AMA_DATABASE_DIR", data_dir);
    path.push("ama.db");
    path
}

fn env_default_path(env_var_name: &str, default: fn() -> Option<PathBuf>) -> PathBuf {
    env::var(env_var_name)
        .map(PathBuf::from)
        .ok()
        .or_else(|| fs_default_path(default))
        .expect("Unable to determine the path.")
}

fn fs_default_path(default: fn() -> Option<PathBuf>) -> Option<PathBuf> {
    default()
        .map(|mut path| {
            path.push("ama");
            path
        })
        .inspect(|path| fs::create_dir_all(path).expect("Failed to create directory"))
}
