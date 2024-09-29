use anyhow::Result;
use clap::Args;

use crate::entities::env_var::EnvVar;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
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
