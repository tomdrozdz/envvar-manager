use anyhow::Result;
use clap::Args;

use crate::entities::env_var::EnvVar;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    value: String,

    /// Do not show the value when listing
    #[clap(short, long)]
    secret: bool,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let env_var = EnvVar::new(self.name.clone(), self.value.clone(), self.secret)?;

        db.env_vars.add(env_var)?;
        Ok(())
    }
}
