use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/01.txt").context("Error reading input file")?;

    let values: Vec<u16> = input
        .lines()
        .map(|line| {
            line.parse::<u16>()
                .with_context(|| format!("Error parsing line {}", line))
        })
        .collect::<Result<_>>()?;

    let increases_part1: u64 = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Part 1: {}", increases_part1);

    let increases_part2: u64 = values
        .iter()
        .tuple_windows()
        .map(|(a, b, c, d)| (a + b + c, b + c + d))
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum();

    println!("Part 2: {}", increases_part2);

    Ok(())
}
