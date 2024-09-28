use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    database_path: PathBuf,
}

impl Config {
    fn default() -> Result<Self> {
        let home = dirs::home_dir().context("Failed to get home directory")?;
        let database_path = home.join(".cache/em.db");

        Ok(Self { database_path })
    }

    fn from_path(config_path: &Path) -> Result<Self> {
        let config_str = std::fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        let config: Config = toml::from_str(&config_str).context("Failed to parse config file")?;
        Ok(config)
    }

    fn to_path(&self, config_path: &Path) -> Result<()> {
        let config_str = toml::to_string_pretty(&self).context("Failed to serialize config")?;
        std::fs::write(config_path, config_str)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))?;

        Ok(())
    }

    pub fn new(config_path: Option<PathBuf>) -> Result<Self> {
        let path = config_path.unwrap_or({
            let home = dirs::home_dir().context("Failed to get home directory")?;
            home.join(".config/em.toml")
        });

        if path.exists() {
            Self::from_path(path.as_path())
        } else {
            let config = Self::default()?;
            config.to_path(path.as_path())?;
            Ok(config)
        }
    }
}
