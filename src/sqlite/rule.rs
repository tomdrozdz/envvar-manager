use std::cell::RefCell;

use anyhow::{anyhow, Result};
use rusqlite::{params, Connection, Error::SqliteFailure};

use crate::entities::rule::Rule;
use crate::repository;

#[derive(Debug)]
pub struct Repository<'a> {
    connection: &'a RefCell<Connection>,
}

impl<'a> Repository<'a> {
    pub fn new(connection: &'a RefCell<Connection>) -> Self {
        Self { connection }
    }
}

impl<'a> repository::Repository<String, Rule> for Repository<'a> {
    fn add(&self, rule: Rule) -> Result<()> {
        let mut binding = self.connection.borrow_mut();
        let transaction = binding.transaction()?;

        transaction
            .execute("INSERT INTO names (name) VALUES (?1)", params![rule.name])
            .map_err(|err| match err {
                SqliteFailure(..) => {
                    anyhow!("Rule or environment variable with the same name already exists")
                }
                _ => err.into(),
            })?;

        transaction.execute(
            "INSERT INTO rules (name, template, updated_at) VALUES (?1, ?2, ?3)",
            params![rule.name, rule.template, rule.updated_at],
        )?;

        transaction.commit()?;
        Ok(())
    }

    fn get(&self, id: &String) -> Result<Rule> {
        let connection = self.connection.borrow();
        let mut stmt =
            connection.prepare("SELECT name, template, updated_at FROM rules WHERE name = ?1")?;

        let mut rows = stmt.query(params![id])?;
        let row = rows.next()?;
        match row {
            Some(row) => {
                let rule = Rule {
                    name: row.get(0)?,
                    template: row.get(1)?,
                    updated_at: row.get(2)?,
                };
                Ok(rule)
            }
            None => Err(anyhow!("Rule not found")),
        }
    }

    fn list(&self) -> Result<Vec<Rule>> {
        let connection = self.connection.borrow();
        let mut stmt = connection.prepare("SELECT name, template, updated_at FROM rules")?;

        let rows = stmt.query_map([], |row| {
            Ok(Rule {
                name: row.get(0)?,
                template: row.get(1)?,
                updated_at: row.get(2)?,
            })
        })?;

        let mut rules = Vec::new();
        for rule in rows {
            rules.push(rule?);
        }

        Ok(rules)
    }

    fn remove(&self, id: &String) -> Result<()> {
        let mut binding = self.connection.borrow_mut();
        let transaction = binding.transaction()?;

        let removed = transaction.execute("DELETE FROM rules WHERE name = ?1", params![id])?;
        if removed == 0 {
            return Err(anyhow!("Rule not found"));
        }

        transaction.execute("DELETE FROM names WHERE name = ?1", params![id])?;
        transaction.commit()?;
        Ok(())
    }

    fn update(&self, rule: Rule) -> Result<()> {
        let connection = self.connection.borrow();

        let updated = connection.execute(
            "UPDATE rules SET template = ?2, updated_at = ?3 WHERE name = ?1",
            params![rule.name, rule.template, rule.updated_at],
        )?;

        if updated == 0 {
            return Err(anyhow!("Rule not found"));
        }

        Ok(())
    }
}
