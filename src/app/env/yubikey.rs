use anyhow::Result;
use clap::Args;

use crate::database::{Database, Repository};
use crate::entities::env_var::EnvVar;
use crate::input::{gui, terminal};

#[derive(Args, Debug)]
pub struct Command {
    /// Do not show a GUI input window, use the terminal instead
    #[clap(short, long)]
    terminal: bool,
}

impl Command {
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        let method = if self.terminal {
            terminal::get_input
        } else {
            gui::get_input
        };

        let yubikey = method("Enter your Yubikey: ")?;
        let name = "YUBIKEY".to_string();

        db.transaction(|transaction| {
            let repo = db.env_vars();

            if let Ok(mut env_var) = repo.get(transaction, &name) {
                env_var.update_value(yubikey);
                repo.update(transaction, env_var)
            } else {
                let env_var = EnvVar::new(name, yubikey, true)?;
                repo.add(transaction, env_var)
            }
        })
    }
}
