use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Template {
    pub name: String,
    pub pattern: String,
    pub updated_at: DateTime<Utc>,
}

impl Template {
    pub fn new(name: String, pattern: String) -> Result<Self> {
        validate_name(&name)?;
        validate_pattern(&pattern)?;

        Ok(Self {
            name,
            pattern,
            updated_at: Utc::now(),
        })
    }

    pub fn update(&mut self, pattern: String) -> Result<()> {
        validate_pattern(&pattern)?;

        self.pattern = pattern;
        self.updated_at = Utc::now();

        Ok(())
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Name cannot be empty"));
    }

    Ok(())
}

fn validate_pattern(pattern: &str) -> Result<()> {
    let mut chars = pattern.chars();
    let mut var_count = 0;

    while let Some(c) = chars.next() {
        if c == '}' {
            return Err(anyhow!("Pattern {pattern} contains unopened braces"));
        }

        if c == '{' {
            let mut length = 0;

            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }

                if c == '{' {
                    return Err(anyhow!("Pattern {pattern} contains nested braces"));
                }

                if !(c.is_ascii_alphanumeric() || c == '_') {
                    return Err(anyhow!(
                        "Variable name in {pattern} pattern contains invalid characters"
                    ));
                }

                length += 1;
            }

            if length == 0 {
                return Err(anyhow!("Pattern {pattern} contains empty braces"));
            }

            var_count += 1;
        }
    }

    if var_count == 0 {
        return Err(anyhow!("Pattern {pattern} does not contain any variables"));
    }

    Ok(())
}
