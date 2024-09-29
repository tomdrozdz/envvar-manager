use anyhow::Result;
use clap::Args;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
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
