use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}
impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, value) = s.splitn(2, ' ').collect_tuple().unwrap();
        let value = value.parse::<u32>()?;

        let instruction = match command {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => return Err(anyhow!("Unknown command: {}", s)),
        };
        Ok(instruction)
    }
}

struct Position {
    x: u32,
    y: u32,
    aim: u32,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/02.txt").context("Error reading input file")?;

    let commands: Vec<Command> = input
        .lines()
        .map(|line| line.parse::<Command>())
        .collect::<Result<_>>()?;

    part1(&commands);
    part2(&commands);

    Ok(())
}

fn part1(commands: &[Command]) {
    let mut position = Position { x: 0, y: 0, aim: 0 };

    for command in commands {
        match command {
            Command::Forward(v) => {
                position.x += v;
            }
            Command::Up(v) => {
                position.y -= v;
            }
            Command::Down(v) => {
                position.y += v;
            }
        }
    }

    println!("Part 1: {}", position.x * position.y);
}

fn part2(commands: &[Command]) {
    let mut position = Position { x: 0, y: 0, aim: 0 };

    for command in commands {
        match command {
            Command::Forward(v) => {
                position.x += v;
                position.y += position.aim * v;
            }
            Command::Up(v) => {
                position.aim -= v;
            }
            Command::Down(v) => {
                position.aim += v;
            }
        }
    }

    println!("Part 2: {}", position.x * position.y);
}
