use anyhow::Result;
use clap::Subcommand;

use crate::sqlite::Database;

mod add;
mod list;
mod remove;
mod update;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new variable template
    Add(add::Command),

    /// List all variable templates
    List(list::Command),

    /// Remove a variable template
    Remove(remove::Command),

    /// Update a variable template
    Update(update::Command),
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        match self {
            Command::Add(add) => add.run(db),
            Command::List(list) => list.run(db),
            Command::Remove(remove) => remove.run(db),
            Command::Update(update) => update.run(db),
        }
    }
}
