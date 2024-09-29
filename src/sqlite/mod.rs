use std::{cell::RefCell, path::PathBuf, rc::Rc};

use anyhow::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::Migrations;

mod env_var;
mod rule;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");

// Define migrations. These are applied atomically.
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

pub fn init_connection(path: &PathBuf) -> Result<Connection> {
    if !path.exists() {
        let directory = path.parent().unwrap();
        std::fs::create_dir_all(directory)?;
    }

    let mut conn = Connection::open(path)?;
    MIGRATIONS.to_latest(&mut conn)?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    Ok(conn)
}

#[derive(Debug)]
pub struct Database {
    pub env_vars: env_var::Repository,
    pub rules: rule::Repository,
}

impl Database {
    pub fn new(conn: Connection) -> Result<Self> {
        let conn_ref = Rc::new(RefCell::new(conn));

        Ok(Self {
            env_vars: env_var::Repository::new(Rc::clone(&conn_ref)),
            rules: rule::Repository::new(conn_ref),
        })
    }
}
