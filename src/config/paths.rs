use std::path::PathBuf;

pub struct ConfigPaths;

impl ConfigPaths {
    pub fn get_config_file_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Current directory config file
        paths.push(PathBuf::from(String::from("./") + env!("CARGO_PKG_NAME") + ".toml"));

        // User config directory
        if let Some(config_dir) = dirs::config_dir() {
            paths.push(config_dir.join(env!("CARGO_PKG_NAME")).join("config.toml"));
        }

        paths
    }
}
