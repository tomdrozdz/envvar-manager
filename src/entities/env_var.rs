use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use tabled::Tabled;

use crate::output::format_date;

#[derive(Debug, Tabled)]
pub struct EnvVar {
    pub name: String,

    #[tabled(display_with("Self::format_value", self))]
    pub value: String,

    pub secret: bool,

    #[tabled(display_with = "format_date")]
    pub updated_at: DateTime<Utc>,
}

impl EnvVar {
    pub fn new(name: String, value: String, secret: bool) -> Result<Self> {
        validate_name(&name)?;

        Ok(Self {
            name,
            value,
            secret,
            updated_at: Utc::now(),
        })
    }

    pub fn update_value(&mut self, value: String) {
        self.value = value;
        self.updated_at = Utc::now();
    }

    pub fn update_secret(&mut self, secret: bool) {
        self.secret = secret;
        self.updated_at = Utc::now();
    }

    fn format_value(&self) -> String {
        if self.secret {
            "<SECRET>".to_string()
        } else {
            self.value.clone()
        }
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Name cannot be empty"));
    }

    Ok(())
}
