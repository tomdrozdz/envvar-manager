use std::{cell::RefCell, path::PathBuf};

use anyhow::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::{Connection, Transaction};
use rusqlite_migration::Migrations;

mod env_var;
mod template;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

#[derive(Debug)]
pub struct Database {
    connection: RefCell<Connection>,
    pub env_vars: env_var::Repository,
    pub templates: template::Repository,
}

impl Database {
    fn from_connection(mut connection: Connection) -> Result<Self> {
        MIGRATIONS.to_latest(&mut connection)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;

        Ok(Self {
            connection: RefCell::new(connection),
            env_vars: env_var::Repository,
            templates: template::Repository,
        })
    }

    pub fn from_path(path: &PathBuf) -> Result<Self> {
        if !path.exists() {
            let directory = path.parent().unwrap();
            std::fs::create_dir_all(directory)?;
        }

        let connection = Connection::open(path)?;
        Self::from_connection(connection)
    }

    pub fn transaction<V>(&self, f: impl FnOnce(&Transaction) -> Result<V>) -> Result<V> {
        let mut connection = self.connection.borrow_mut();
        let transaction = connection.transaction()?;

        let result = f(&transaction);
        match result {
            Ok(_) => transaction.commit(),
            Err(_) => transaction.rollback(),
        }?;

        result
    }
}

#[cfg(test)]
impl Database {
    pub fn in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Self::from_connection(connection)
    }
}
