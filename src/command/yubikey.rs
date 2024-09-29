use anyhow::Result;
use clap::Parser;

use crate::entities::env_var::EnvVar;
use crate::gui::get_input;
use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let yubikey = get_input("Enter your Yubikey: ")?;
        let name = "YUBIKEY".to_string();

        if let Ok(mut env_var) = db.env_vars.get(&name) {
            env_var.update(yubikey.clone());
            db.env_vars.update(env_var)?;
        } else {
            let env_var = EnvVar::new(name, yubikey.clone())?;
            db.env_vars.add(env_var)?;
        };

        Ok(())
    }
}
