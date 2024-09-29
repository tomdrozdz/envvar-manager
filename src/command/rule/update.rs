use anyhow::Result;
use clap::Parser;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {
    name: String,
    template: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let mut rule = db.rules.get(&self.name)?;
        rule.update(self.template.clone())?;

        db.rules.update(rule)?;
        Ok(())
    }
}
