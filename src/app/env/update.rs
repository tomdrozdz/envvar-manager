use anyhow::Result;
use clap::{ArgGroup, Args};

use crate::repository::Repository;
use crate::sqlite::Database;

#[derive(Args, Debug)]
#[clap(group(
    ArgGroup::new("fields")
        .required(true)
        .multiple(true)
        .args(&["value", "secret"]),
))]
pub struct Command {
    name: String,

    #[clap(group = "fields")]
    value: Option<String>,

    /// Change the secret status
    #[clap(short, long, group = "fields")]
    secret: Option<bool>,
}

impl Command {
    pub fn run(&self, db: &Database) -> Result<()> {
        db.transaction(|transaction| {
            let mut env_var = db.env_vars.get(transaction, &self.name)?;

            if let Some(value) = &self.value {
                env_var.update_value(value.clone());
            }

            if let Some(secret) = self.secret {
                env_var.update_secret(secret);
            }

            db.env_vars.update(transaction, env_var)
        })
    }
}
