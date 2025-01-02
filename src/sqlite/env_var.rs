use anyhow::{anyhow, Result};
use rusqlite::{params, Error::SqliteFailure, Transaction};

use crate::{entities::env_var::EnvVar, repository};

#[derive(Debug, Default)]
pub struct Repository;

impl repository::Repository<String, EnvVar, Transaction<'_>> for Repository {
    fn add(&self, transaction: &Transaction, env_var: EnvVar) -> Result<()> {
        transaction
            .execute(
                "INSERT INTO names (name) VALUES (?1)",
                params![env_var.name],
            )
            .map_err(|err| match err {
                SqliteFailure(..) => {
                    anyhow!(
                        "Environment variable or template {} already exists",
                        env_var.name
                    )
                }
                _ => err.into(),
            })?;

        transaction.execute(
            "INSERT INTO env_vars (name, value, secret, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                env_var.name,
                env_var.value,
                env_var.secret,
                env_var.updated_at
            ],
        )?;

        Ok(())
    }

    fn get(&self, transaction: &Transaction, name: &String) -> Result<EnvVar> {
        let mut stmt = transaction
            .prepare("SELECT name, value, secret, updated_at FROM env_vars WHERE name = ?1")?;

        let mut rows = stmt.query(params![name])?;
        let row = rows.next()?;
        match row {
            Some(row) => Ok(EnvVar {
                name: row.get(0)?,
                value: row.get(1)?,
                secret: row.get(2)?,
                updated_at: row.get(3)?,
            }),
            None => Err(anyhow!("Environment variable {name} not found")),
        }
    }

    fn list(&self, transaction: &Transaction) -> Result<Vec<EnvVar>> {
        let mut stmt =
            transaction.prepare("SELECT name, value, secret, updated_at FROM env_vars")?;

        let rows = stmt.query_map([], |row| {
            Ok(EnvVar {
                name: row.get(0)?,
                value: row.get(1)?,
                secret: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?;

        let env_vars = rows.collect::<Result<_, _>>()?;
        Ok(env_vars)
    }

    fn remove(&self, transaction: &Transaction, name: &String) -> Result<()> {
        let removed = transaction.execute("DELETE FROM env_vars WHERE name = ?1", params![name])?;
        if removed == 0 {
            return Err(anyhow!("Environment variable {name} not found"));
        }

        transaction.execute("DELETE FROM names WHERE name = ?1", params![name])?;
        Ok(())
    }

    fn update(&self, transaction: &Transaction, env_var: EnvVar) -> Result<()> {
        let updated = transaction.execute(
            "UPDATE env_vars SET value = ?2, secret = ?3, updated_at = ?4 WHERE name = ?1",
            params![
                env_var.name,
                env_var.value,
                env_var.secret,
                env_var.updated_at
            ],
        )?;

        if updated == 0 {
            return Err(anyhow!("Environment variable {} not found", env_var.name));
        }

        Ok(())
    }
}
