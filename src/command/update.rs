use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Command {
    name: String,
    value: String,
}

impl Command {
    pub fn run(&self) -> Result<()> {
        println!("Export command");
        Ok(())
    }
}
