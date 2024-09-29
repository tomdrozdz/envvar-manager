use std::{cell::RefCell, path::PathBuf};

use anyhow::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use rusqlite::Connection;
use rusqlite_migration::Migrations;

mod env_var;
mod rule;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

pub fn init_connection(path: &PathBuf) -> Result<RefCell<Connection>> {
    if !path.exists() {
        let directory = path.parent().unwrap();
        std::fs::create_dir_all(directory)?;
    }

    let mut connection = Connection::open(path)?;
    MIGRATIONS.to_latest(&mut connection)?;
    connection.pragma_update(None, "foreign_keys", "ON")?;

    Ok(RefCell::new(connection))
}

#[derive(Debug)]
pub struct Database<'a> {
    pub env_vars: env_var::Repository<'a>,
    pub rules: rule::Repository<'a>,
}

impl<'a> Database<'a> {
    pub fn new(connection: &'a RefCell<Connection>) -> Self {
        Self {
            env_vars: env_var::Repository::new(connection),
            rules: rule::Repository::new(connection),
        }
    }
}
