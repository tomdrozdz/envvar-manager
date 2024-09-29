use anyhow::Result;

pub trait Repository<K, V> {
    fn add(&self, entity: V) -> Result<()>;
    fn get(&self, id: &K) -> Result<V>;
    fn list(&self) -> Result<Vec<V>>;
    fn remove(&self, id: &K) -> Result<()>;
    fn update(&self, entity: V) -> Result<()>;
}
