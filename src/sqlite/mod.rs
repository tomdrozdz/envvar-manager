use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use rusqlite::Connection;

use crate::config::Config;

mod env_var;
mod rule;

#[derive(Debug)]
pub struct Database {
    pub env_vars: env_var::Repository,
    pub rules: rule::Repository,
}

impl Database {
    pub fn new(config: &Config) -> Result<Self> {
        if !config.database_path.exists() {
            let directory = config.database_path.parent().unwrap();
            std::fs::create_dir_all(directory)?;
        }

        let conn = Connection::open(&config.database_path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS names (
                name TEXT PRIMARY KEY
            );

            CREATE TABLE IF NOT EXISTS env_vars (
                name TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(name) REFERENCES names(name) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS rules (
                name TEXT PRIMARY KEY,
                template TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY(name) REFERENCES names(name) ON DELETE CASCADE
            );
            ",
        )?;

        let conn_ref = Rc::new(RefCell::new(conn));
        Ok(Self {
            env_vars: env_var::Repository::new(Rc::clone(&conn_ref)),
            rules: rule::Repository::new(conn_ref),
        })
    }
}
