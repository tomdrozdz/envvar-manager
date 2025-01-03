use anyhow::Result;

use crate::entities::{env_var::EnvVar, template::Template};

pub trait Repository<K, V, T> {
    fn add(&self, transaction: &T, entity: V) -> Result<()>;
    fn get(&self, transaction: &T, id: &K) -> Result<V>;
    fn list(&self, transaction: &T) -> Result<Vec<V>>;
    fn remove(&self, transaction: &T, id: &K) -> Result<()>;
    fn update(&self, transaction: &T, entity: V) -> Result<()>;
}

pub trait Database<T> {
    type EnvVarRepository: Repository<String, EnvVar, T>;
    type TemplateRepository: Repository<String, Template, T>;

    fn env_vars(&self) -> &Self::EnvVarRepository;
    fn templates(&self) -> &Self::TemplateRepository;
    fn transaction<V, F>(&self, f: F) -> Result<V>
    where
        F: FnOnce(&T) -> Result<V>;
}
