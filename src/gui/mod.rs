use anyhow::Result;

pub fn get_input(prompt: &str) -> Result<String> {
    use std::io::{self, Write};

    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(input.trim().to_string())
}
