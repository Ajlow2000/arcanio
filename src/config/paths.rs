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

    pub fn get_user_config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join(env!("CARGO_PKG_NAME")))
    }

    pub fn ensure_user_config_dir() -> crate::Result<PathBuf> {
        let config_dir = Self::get_user_config_dir()
            .ok_or_else(|| crate::Error::ConfigLoadError("Unable to determine user config directory".to_string()))?;
        
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }
        
        Ok(config_dir)
    }
}
