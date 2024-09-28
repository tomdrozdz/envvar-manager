use clap::Parser;

#[derive(Parser)]
pub struct Command {}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("Export command");
        Ok(())
    }
}
