use anyhow::Result;
use clap::Subcommand;

use crate::sqlite::Database;

mod add;
mod list;
mod remove;
mod update;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new templating rule
    Add(add::Command),

    /// List all templating rules
    List(list::Command),

    /// Remove a templating rule
    Remove(remove::Command),

    /// Update a templating rule
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
