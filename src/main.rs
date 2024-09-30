use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use config::Config;
use std::path::PathBuf;

mod app;
mod config;
mod entities;
mod input;
mod repository;
mod resolver;
mod sqlite;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(flatten)]
    AppCommand(app::Command),

    /// Generate shell completions
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::AppCommand(command) => {
            let config = Config::new(cli.config)?;
            command.run(&config)
        }
        Command::Completion { shell } => {
            let command = &mut Cli::command();
            generate(
                shell,
                command,
                command.get_name().to_string(),
                &mut std::io::stdout(),
            );
            Ok(())
        }
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
