use std::cmp::Ordering;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = aoc_2021::read_input("03")?;

    let ones = count_ones(&input);
    let gamma = ones
        .iter()
        .map(|&count| {
            if count as usize >= input.len() / 2 {
                1
            } else {
                0
            }
        })
        .join("");
    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = !gamma & (2u32.pow(ones.len() as u32) - 1);

    println!("Part 1: {:?}", gamma * epsilon);

    let oxygen = u32::from_str_radix(&filter_list(input.clone(), 1), 2)?;
    let co2 = u32::from_str_radix(&filter_list(input, 0), 2)?;
    println!("Part 2: {}", oxygen * co2);

    Ok(())
}

fn count_ones(numbers: &[String]) -> Vec<u32> {
    numbers
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .reduce(|accum: Vec<u32>, item| {
            accum
                .iter()
                .zip(item.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        })
        .unwrap()
}

fn filter_list(mut list: Vec<String>, preferred: u32) -> String {
    for i in 0.. {
        let num_ones = count_ones(&list)[i];
        let to_keep = match num_ones.cmp(&(list.len() as u32 - num_ones)) {
            Ordering::Less => {
                if preferred == 1 {
                    '0'
                } else {
                    '1'
                }
            }
            Ordering::Equal => preferred.to_string().chars().next().unwrap(),
            Ordering::Greater => preferred.to_string().chars().next().unwrap(),
        };
        list = list
            .into_iter()
            .filter(|item| item.chars().nth(i).unwrap() == to_keep)
            .collect();
        if list.len() == 1 {
            break;
        }
    }
    list[0].clone()
}
