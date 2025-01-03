use anyhow::Result;
use clap::Subcommand;

use crate::config::Config;
use crate::sqlite;

mod env;
mod export;
mod template;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage environment variables
    #[clap(subcommand)]
    Env(env::Command),

    /// Write export statements with all saved environment variables to stdout
    Export(export::Command),

    /// Manage variable templates
    #[clap(subcommand)]
    Template(template::Command),
}

impl Command {
    pub fn run(&self, config: &Config) -> Result<()> {
        log::debug!("Running command {:?} with config {:?}", self, config);
        let db = sqlite::Database::from_path(&config.database_path)?;

        match self {
            Command::Env(env) => env.run(&db),
            Command::Export(export) => export.run(&db),
            Command::Template(template) => template.run(&db),
        }
    }
}
