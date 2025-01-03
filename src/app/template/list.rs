use anyhow::Result;
use clap::Args;
use tabled::settings::peaker::PriorityMax;
use tabled::settings::{Style, Width};
use tabled::Table;

use crate::database::{Database, Repository};
use crate::output::get_terminal_size;

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        let templates = db.transaction(|transaction| db.templates().list(transaction))?;

        let mut table = Table::new(templates);
        let (width, _) = get_terminal_size();

        table.with(Style::modern());
        table.with(Width::wrap(width).priority(PriorityMax));

        println!("{table}");
        Ok(())
    }
}
