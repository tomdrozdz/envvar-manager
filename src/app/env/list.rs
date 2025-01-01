use anyhow::Result;
use clap::Args;
use tabled::{
    settings::{peaker::PriorityMax, Style, Width},
    Table,
};

use crate::sqlite::Database;
use crate::{output::get_terminal_size, repository::Repository};

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let env_vars = db.env_vars.list()?;

        let mut table = Table::new(env_vars);
        let (width, _) = get_terminal_size();

        table.with(Style::modern());
        table.with(Width::wrap(width).priority(PriorityMax));

        println!("{table}");
        Ok(())
    }
}
