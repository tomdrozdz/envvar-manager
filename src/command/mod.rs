use anyhow::Result;
use clap::Subcommand;

use crate::config::Config;
use crate::sqlite::Database;

mod add;
mod export;
mod list;
mod remove;
mod rule;
mod update;
mod yubikey;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new environment variable
    Add(add::Command),

    /// Write export statements with all saved environment variables to stdout
    Export(export::Command),

    /// List all saved environment variables
    List(list::Command),

    // Remove an environment variable
    Remove(remove::Command),

    /// Update an environment variable
    Update(update::Command),

    /// Update the Yubikey value via a GUI input window
    Yubikey(yubikey::Command),

    /// Manage templating rules
    #[clap(subcommand)]
    Rule(rule::Command),
}

impl Command {
    pub fn run(&self, config: &Config) -> Result<()> {
        log::debug!("Running command {:?} with config {:?}", self, config);
        let db = Database::new(config)?;

        match self {
            Command::Add(add) => add.run(&db),
            Command::Export(export) => export.run(&db),
            Command::List(list) => list.run(&db),
            Command::Remove(remove) => remove.run(&db),
            Command::Update(update) => update.run(&db),
            Command::Yubikey(yubikey) => yubikey.run(&db),
            Command::Rule(rule) => rule.run(&db),
        }
    }
}
