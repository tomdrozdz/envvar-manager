use anyhow::Result;
use clap::Args;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    pattern: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        db.transaction(|transaction| {
            let mut template = db.templates.get(transaction, &self.name)?;
            template.update_pattern(self.pattern.clone())?;
            db.templates.update(transaction, template)
        })
    }
}
