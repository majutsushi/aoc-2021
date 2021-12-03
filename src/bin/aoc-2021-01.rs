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

    let increases_part1: usize = values.iter().tuple_windows().filter(|(a, b)| b > a).count();

    println!("Part 1: {}", increases_part1);

    let increases_part2: usize = values
        .iter()
        .tuple_windows()
        .map(|(a, b, c, d)| (a + b + c, b + c + d))
        .filter(|(a, b)| b > a)
        .count();

    println!("Part 2: {}", increases_part2);

    Ok(())
}
