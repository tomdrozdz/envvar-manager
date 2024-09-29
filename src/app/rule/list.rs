use anyhow::Result;
use clap::Args;
use tabled::Table;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let rules = db.rules.list()?;

        println!("{}", Table::new(rules));
        Ok(())
    }
}
