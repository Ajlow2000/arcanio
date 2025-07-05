use crate::{cli::ConfigCommand, config::AppConfig, Result};

#[tracing::instrument]
pub async fn handle_config(config_cmd: ConfigCommand, current_config: &AppConfig) -> Result<()> {
    match config_cmd {
        ConfigCommand::Default => handle_config_default().await,
        ConfigCommand::Current => handle_config_current(current_config).await,
    }
}

#[tracing::instrument]
pub async fn handle_config_default() -> Result<()> {
    let default_config = AppConfig::default();
    let toml_string = toml::to_string_pretty(&default_config)
        .map_err(|e| crate::Error::ConfigSerializationError(format!("Failed to serialize default config: {}", e)))?;
    
    println!("{}", toml_string);
    Ok(())
}

#[tracing::instrument]
pub async fn handle_config_current(config: &AppConfig) -> Result<()> {
    let toml_string = toml::to_string_pretty(config)
        .map_err(|e| crate::Error::ConfigSerializationError(format!("Failed to serialize current config: {}", e)))?;
    
    println!("{}", toml_string);
    Ok(())
}
