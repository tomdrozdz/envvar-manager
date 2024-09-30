use anyhow::Result;
use clap::Args;

use crate::entities::template::Template;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    pattern: String,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let template = Template::new(self.name.clone(), self.pattern.clone())?;

        db.templates.add(template)?;
        Ok(())
    }
}
