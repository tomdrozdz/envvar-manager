use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

impl EnvVar {
    pub fn new(name: String, value: String) -> Result<Self> {
        validate_name(&name)?;

        Ok(Self {
            name,
            value,
            updated_at: Utc::now(),
        })
    }

    pub fn update(&mut self, value: String) {
        self.value = value;
        self.updated_at = Utc::now();
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Name cannot be empty"));
    }

    Ok(())
}
