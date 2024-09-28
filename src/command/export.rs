use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Command {
    no_export: bool,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        println!("Export command");
        Ok(())
    }
}
