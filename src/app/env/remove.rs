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
        db.env_vars.remove(&self.name)?;
        Ok(())
    }
}
