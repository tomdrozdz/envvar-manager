use std::collections::HashMap;

use crate::entities::{env_var::EnvVar, rule::Rule};
use anyhow::{Context, Result};

fn resolve_rule(
    sources: &HashMap<&str, &str>,
    name: String,
    template: &str,
) -> Result<(String, String)> {
    let mut chars = template.chars();
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut template_var = String::new();
            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }
                template_var.push(c);
            }

            let value = sources.get(template_var.as_str()).with_context(|| {
                format!("Could not resolve variable {template_var} from rule {name}")
            })?;

            result.push_str(value);
        } else {
            result.push(c);
        }
    }

    Ok((name, result))
}

pub fn resolve(env_vars: &[EnvVar], rules: &[Rule]) -> Result<Vec<(String, String)>> {
    let sources: HashMap<_, _> = env_vars
        .iter()
        .map(|env_var| (env_var.name.as_str(), env_var.value.as_str()))
        .collect();

    let resolved: Vec<_> = rules
        .iter()
        .map(|rule| resolve_rule(&sources, rule.name.clone(), rule.template.as_str()))
        .collect::<Result<_>>()?;

    Ok(resolved)
}
