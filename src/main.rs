use anyhow::Result;
use clap::Parser;
use config::Config;
use std::path::PathBuf;

mod command;
mod config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: command::Command,
}

fn run(cli: Cli) -> Result<()> {
    let config = Config::new(cli.config)?;
    cli.command.run(&config)
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
