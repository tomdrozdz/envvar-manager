use anyhow::Result;
use clap::Args;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        db.transaction(|transaction| db.env_vars.remove(transaction, &self.name))
    }
}
