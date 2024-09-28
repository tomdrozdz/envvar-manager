use clap::Parser;

#[derive(Parser)]
pub struct Command {
    no_export: bool,
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("Export command");
        Ok(())
    }
}
