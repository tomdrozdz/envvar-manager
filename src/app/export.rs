use std::io::{BufWriter, Write};

use anyhow::Result;
use clap::Args;

use crate::repository::Repository;
use crate::{resolver::resolve, sqlite::Database};

#[derive(Args, Debug)]
pub struct Command {}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        let env_vars = db.env_vars.list()?;
        let rules = db.rules.list()?;

        let rule_vars = resolve(&env_vars, &rules)?;

        let stdout = std::io::stdout();
        let lock = stdout.lock();
        let mut writer = BufWriter::new(lock);

        for env_var in env_vars {
            writeln!(writer, "export {}='{}'", env_var.name, env_var.value)?;
        }
        for (name, value) in rule_vars {
            writeln!(writer, "export {name}='{value}'")?;
        }

        Ok(())
    }
}
