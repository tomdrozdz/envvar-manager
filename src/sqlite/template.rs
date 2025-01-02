use anyhow::{anyhow, Result};
use rusqlite::{params, Error::SqliteFailure};
use rusqlite::{Row, Transaction};

use crate::entities::template::Template;
use crate::repository;

#[derive(Debug, Default)]
pub struct Repository;

fn from_row(row: &Row) -> Result<Template, rusqlite::Error> {
    Ok(Template {
        name: row.get(0)?,
        pattern: row.get(1)?,
        updated_at: row.get(2)?,
    })
}

macro_rules! to_row {
    ($template:expr) => {
        params![$template.name, $template.pattern, $template.updated_at]
    };
}

impl repository::Repository<String, Template, Transaction<'_>> for Repository {
    fn add(&self, transaction: &Transaction, template: Template) -> Result<()> {
        transaction
            .execute(
                "INSERT INTO names (name) VALUES (?1)",
                params![template.name],
            )
            .map_err(|err| match err {
                SqliteFailure(..) => {
                    anyhow!(
                        "Template or environment variable {} already exists",
                        template.name
                    )
                }
                _ => err.into(),
            })?;

        transaction.execute(
            "INSERT INTO templates (name, pattern, updated_at) VALUES (?1, ?2, ?3)",
            to_row!(template),
        )?;

        Ok(())
    }

    fn get(&self, transaction: &Transaction, name: &String) -> Result<Template> {
        let mut stmt = transaction
            .prepare("SELECT name, pattern, updated_at FROM templates WHERE name = ?1")?;

        let mut rows = stmt.query(params![name])?;
        let row = rows.next()?;
        match row {
            Some(row) => Ok(from_row(row)?),
            None => Err(anyhow!("Template {name} not found")),
        }
    }

    fn list(&self, transaction: &Transaction) -> Result<Vec<Template>> {
        let mut stmt = transaction.prepare("SELECT name, pattern, updated_at FROM templates")?;

        let rows = stmt.query_map([], from_row)?;

        let mut templates = Vec::new();
        for template in rows {
            templates.push(template?);
        }

        Ok(templates)
    }

    fn remove(&self, transaction: &Transaction, name: &String) -> Result<()> {
        let removed =
            transaction.execute("DELETE FROM templates WHERE name = ?1", params![name])?;
        if removed == 0 {
            return Err(anyhow!("Template {name} not found"));
        }

        transaction.execute("DELETE FROM names WHERE name = ?1", params![name])?;
        Ok(())
    }

    fn update(&self, transaction: &Transaction, template: Template) -> Result<()> {
        let updated = transaction.execute(
            "UPDATE templates SET pattern = ?2, updated_at = ?3 WHERE name = ?1",
            to_row!(template),
        )?;

        if updated == 0 {
            return Err(anyhow!("Template {} not found", template.name));
        }

        Ok(())
    }
}
