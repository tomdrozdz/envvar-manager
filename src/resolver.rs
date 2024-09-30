use std::collections::HashMap;

use crate::entities::{env_var::EnvVar, template::Template};
use anyhow::{Context, Result};

fn resolve_template(
    sources: &HashMap<&str, &str>,
    name: String,
    pattern: &str,
) -> Result<(String, String)> {
    let mut chars = pattern.chars();
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut pattern_var = String::new();
            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }
                pattern_var.push(c);
            }

            let value = sources.get(pattern_var.as_str()).with_context(|| {
                format!("Could not resolve variable {pattern_var} from template {name}")
            })?;

            result.push_str(value);
        } else {
            result.push(c);
        }
    }

    Ok((name, result))
}

pub fn resolve(env_vars: &[EnvVar], templates: &[Template]) -> Result<Vec<(String, String)>> {
    let sources: HashMap<_, _> = env_vars
        .iter()
        .map(|env_var| (env_var.name.as_str(), env_var.value.as_str()))
        .collect();

    let resolved: Vec<_> = templates
        .iter()
        .map(|template| {
            resolve_template(&sources, template.name.clone(), template.pattern.as_str())
        })
        .collect::<Result<_>>()?;

    Ok(resolved)
}
