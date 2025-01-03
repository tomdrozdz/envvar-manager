use anyhow::Result;
use clap::Args;

use crate::database::{Database, Repository};

#[derive(Args, Debug)]
pub struct Command {
    name: String,
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        db.transaction(|transaction| db.templates().remove(transaction, &self.name))
    }
}
