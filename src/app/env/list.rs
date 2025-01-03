use anyhow::Result;
use clap::Args;
use tabled::{
    settings::{peaker::PriorityMax, Style, Width},
    Table,
};

use crate::database::{Database, Repository};
use crate::output::get_terminal_size;

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        let env_vars = db.transaction(|transaction| db.env_vars().list(transaction))?;

        let mut table = Table::new(env_vars);
        let (width, _) = get_terminal_size();

        table.with(Style::modern());
        table.with(Width::wrap(width).priority(PriorityMax));

        println!("{table}");
        Ok(())
    }
}
