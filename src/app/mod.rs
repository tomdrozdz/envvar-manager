use anyhow::Result;
use clap::Subcommand;

use crate::config::Config;
use crate::sqlite::{init_connection, Database};

mod env;
mod export;
mod rule;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage environment variables
    #[clap(subcommand)]
    Env(env::Command),

    /// Write export statements with all saved environment variables to stdout
    Export(export::Command),

    /// Manage templating rules
    #[clap(subcommand)]
    Rule(rule::Command),
}

impl Command {
    pub fn run(&self, config: &Config) -> Result<()> {
        log::debug!("Running command {:?} with config {:?}", self, config);
        let db_connection = init_connection(&config.database_path)?;
        let db = Database::new(&db_connection);

        match self {
            Command::Env(env) => env.run(&db),
            Command::Export(export) => export.run(&db),
            Command::Rule(rule) => rule.run(&db),
        }
    }
}
