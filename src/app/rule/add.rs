use anyhow::Result;
use clap::Args;

use crate::entities::rule::Rule;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    template: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let rule = Rule::new(self.name.clone(), self.template.clone())?;

        db.rules.add(rule)?;
        Ok(())
    }
}
