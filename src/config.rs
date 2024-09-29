use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "kebab-case")]
pub struct Config {
    pub database_path: PathBuf,
}

impl Config {
    fn from_path(config_path: &Path) -> Result<Self> {
        let config_str =
            std::fs::read_to_string(config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn new(config_path: Option<PathBuf>) -> Result<Self> {
        let path = config_path.unwrap_or({
            let home = dirs::home_dir().expect("Failed to get home directory");
            home.join(".config/envvar-manager/config.toml")
        });

        let path = std::path::absolute(path)?;
        log::debug!("Using config path: {}", path.display());

        if !path.exists() {
            log::warn!(
                "Config file {} does not exist, using default config",
                path.display()
            );

            return Ok(Config::default());
        }

        Self::from_path(path.as_path())
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().expect("Failed to get home directory");
        let database_path = home.join(".cache/envvar-manager/db.sqlite");

        Self { database_path }
    }
}
