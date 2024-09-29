use anyhow::Result;
use clap::Parser;

use crate::entities::env_var::EnvVar;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {
    name: String,
    value: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let env_var = EnvVar::new(self.name.clone(), self.value.clone())?;

        db.env_vars.add(env_var)?;
        Ok(())
    }
}
