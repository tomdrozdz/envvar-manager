use anyhow::Result;
use clap::Args;

use crate::entities::env_var::EnvVar;
use crate::input::{gui, terminal};
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
pub struct Command {
    /// Do not show a GUI input window, use the terminal instead
    #[clap(short, long)]
    terminal: bool,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let method = if self.terminal {
            terminal::get_input
        } else {
            gui::get_input
        };

        let yubikey = method("Enter your Yubikey: ")?;
        let name = "YUBIKEY".to_string();

        if let Ok(mut env_var) = db.env_vars.get(&name) {
            env_var.update(yubikey);
            db.env_vars.update(env_var)?;
        } else {
            let env_var = EnvVar::new(name, yubikey)?;
            db.env_vars.add(env_var)?;
        };

        Ok(())
    }
}
