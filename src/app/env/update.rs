use anyhow::Result;
use clap::{ArgGroup, Args};

use crate::database::{Database, Repository};

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
    pub fn run<T, D: Database<T>>(&self, db: &D) -> Result<()> {
        db.transaction(|transaction| {
            let repo = db.env_vars();
            let mut env_var = repo.get(transaction, &self.name)?;

            if let Some(value) = &self.value {
                env_var.update_value(value.clone());
            }

            if let Some(secret) = self.secret {
                env_var.update_secret(secret);
            }

            repo.update(transaction, env_var)
        })
    }
}
