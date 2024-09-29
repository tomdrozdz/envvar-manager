use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Rule {
    pub name: String,
    pub template: String,
    pub updated_at: DateTime<Utc>,
}

impl Rule {
    pub fn new(name: String, template: String) -> Result<Self> {
        validate_name(&name)?;
        validate_template(&template)?;

        Ok(Self {
            name,
            template,
            updated_at: Utc::now(),
        })
    }

    pub fn update(&mut self, template: String) -> Result<()> {
        validate_template(&template)?;

        self.template = template;
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

fn validate_template(template: &str) -> Result<()> {
    let mut chars = template.chars();
    let mut var_count = 0;

    while let Some(c) = chars.next() {
        if c == '}' {
            return Err(anyhow!("Template {template} contains unopened braces"));
        }

        if c == '{' {
            let mut length = 0;

            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }

                if c == '{' {
                    return Err(anyhow!("Template {template} contains nested braces"));
                }

                if !c.is_ascii_alphanumeric() {
                    return Err(anyhow!(
                        "Variable name in {template} template contains invalid characters"
                    ));
                }

                length += 1;
            }

            if length == 0 {
                return Err(anyhow!("Template {template} contains empty braces"));
            }

            var_count += 1;
        }
    }

    if var_count == 0 {
        return Err(anyhow!(
            "Template {template} does not contain any variables"
        ));
    }

    Ok(())
}
