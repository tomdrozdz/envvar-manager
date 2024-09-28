use clap::Parser;

#[derive(Parser)]
pub struct Command {
    name: String,
    value: String,
}

impl Command {
    pub fn run(&self) -> anyhow::Result<()> {
        println!("Export command");
        Ok(())
    }
}
