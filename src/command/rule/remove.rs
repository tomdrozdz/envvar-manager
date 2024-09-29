use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {
    name: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        db.rules.remove(&self.name)?;
        Ok(())
    }
}
