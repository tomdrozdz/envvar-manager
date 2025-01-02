use anyhow::Result;

pub trait Repository<K, V, T> {
    fn add(&self, transaction: &T, entity: V) -> Result<()>;
    fn get(&self, transaction: &T, id: &K) -> Result<V>;
    fn list(&self, transaction: &T) -> Result<Vec<V>>;
    fn remove(&self, transaction: &T, id: &K) -> Result<()>;
    fn update(&self, transaction: &T, entity: V) -> Result<()>;
}
