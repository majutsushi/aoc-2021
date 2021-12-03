use anyhow::{Context, Result};

pub fn read_input(day: &str) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(format!("input/{}.txt", day))
        .context("Error reading input file")?;
    Ok(text.lines().map(str::to_string).collect())
}
