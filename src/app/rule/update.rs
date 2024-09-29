use anyhow::Result;
use clap::Args;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
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
