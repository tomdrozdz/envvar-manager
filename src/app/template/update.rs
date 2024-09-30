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
        let mut template = db.templates.get(&self.name)?;
        template.update(self.pattern.clone())?;

        db.templates.update(template)?;
        Ok(())
    }
}
