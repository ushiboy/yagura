use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspace_root: Option<String>,
    pub commands: Vec<CommandConfig>,
}

#[derive(Debug, Deserialize)]
pub struct CommandConfig {
    pub command: String,
    pub working_dir: Option<String>,
}

pub fn load_config(path: PathBuf) -> anyhow::Result<Config> {
    let config_str = std::fs::read_to_string(path)?;
    let config: Config = serde_saphyr::from_str(&config_str)?;
    Ok(config)
}
