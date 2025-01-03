use anyhow::Result;
use clap::Subcommand;

use crate::database::Database;

mod add;
mod list;
mod remove;
mod update;
mod yubikey;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new environment variable
    Add(add::Command),

    /// List all saved environment variables
    List(list::Command),

    /// Remove an environment variable
    Remove(remove::Command),

    /// Update an environment variable
    Update(update::Command),

    /// Update the YUBIKEY environment variable via an input method
    Yubikey(yubikey::Command),
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        match self {
            Command::Add(add) => add.run(db),
            Command::List(list) => list.run(db),
            Command::Remove(remove) => remove.run(db),
            Command::Update(update) => update.run(db),
            Command::Yubikey(yubikey) => yubikey.run(db),
        }
    }
}
