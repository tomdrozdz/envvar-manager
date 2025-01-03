use anyhow::Result;
use clap::Args;

use crate::database::{Database, Repository};
use crate::entities::template::Template;

#[derive(Args, Debug)]
pub struct Command {
    name: String,
    pattern: String,
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        let template = Template::new(self.name.clone(), self.pattern.clone())?;
        db.transaction(|transaction| db.templates().add(transaction, template))
    }
}
