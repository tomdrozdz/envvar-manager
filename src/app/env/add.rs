use anyhow::Result;
use clap::Args;

use crate::database::{Database, Repository};
use crate::entities::env_var::EnvVar;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    value: String,

    /// Do not show the value when listing
    #[clap(short, long)]
    secret: bool,
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        let env_var = EnvVar::new(self.name.clone(), self.value.clone(), self.secret)?;
        db.transaction(|transaction| db.env_vars().add(transaction, env_var))
    }
}
