use anyhow::Result;
use clap::Args;

use crate::database::{Database, Repository};

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    pattern: String,
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        db.transaction(|transaction| {
            let repo = db.templates();
            let mut template = repo.get(transaction, &self.name)?;
            template.update_pattern(self.pattern.clone())?;
            repo.update(transaction, template)
        })
    }
}
