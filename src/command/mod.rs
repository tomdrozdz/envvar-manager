use clap::Subcommand;

use crate::config::Config;

mod export;
mod update;
mod yubikey;

#[derive(Subcommand)]
pub enum Command {
    /// Return all the saved environment variables to stdout
    Export(export::Command),

    /// Update an environment variable
    Update(update::Command),

    /// Update the Yubikey value via a GUI input window
    Yubikey(yubikey::Command),
}

impl Command {
    pub fn run(&self, config: &Config) -> anyhow::Result<()> {
        println!("Running command: {config:?}");

        match self {
            Command::Export(export) => export.run(),
            Command::Update(update) => update.run(),
            Command::Yubikey(yubikey) => yubikey.run(),
        }
    }
}
