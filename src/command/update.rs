use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {
    name: String,
    value: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let mut env_var = db.env_vars.get(&self.name)?;
        env_var.update(self.value.clone());

        db.env_vars.update(env_var)?;
        Ok(())
    }
}
