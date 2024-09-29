use anyhow::Result;
use clap::Parser;
use tabled::Table;

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let env_vars = db.env_vars.list()?;

        println!("{}", Table::new(env_vars));
        Ok(())
    }
}
