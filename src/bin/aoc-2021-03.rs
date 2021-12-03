use std::cmp::Ordering;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = aoc_2021::read_input("03")?;

    let num_len = input[0].len();
    let numbers: Vec<u32> = input
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect_vec();

    let mut gamma = 0u32;
    for i in 0..num_len {
        let mask = 2u32.pow(i as u32);
        let num_ones = count_ones(&numbers, mask);
        if num_ones >= input.len() / 2 {
            gamma += mask;
        }
    }

    let epsilon = !gamma & (2u32.pow(num_len as u32) - 1);

    println!("Part 1: {}", gamma * epsilon);

    let oxygen = filter_numbers(&numbers, num_len, 1);
    let co2 = filter_numbers(&numbers, num_len, 0);
    println!("Part 2: {}", oxygen * co2);

    Ok(())
}

fn count_ones(numbers: &[u32], mask: u32) -> usize {
    numbers.iter().filter(|&&num| num & mask != 0).count()
}

fn filter_numbers(numbers: &[u32], num_len: usize, preferred: u32) -> u32 {
    let mut numbers = numbers.to_owned();
    for i in (0..num_len).rev() {
        let mask = 2u32.pow(i as u32);
        let num_ones = count_ones(&numbers, mask);
        let to_keep = match num_ones.cmp(&(numbers.len() - num_ones)) {
            Ordering::Less => {
                if preferred == 1 {
                    0
                } else {
                    1
                }
            }
            Ordering::Equal => preferred,
            Ordering::Greater => preferred,
        };
        numbers = numbers
            .into_iter()
            .filter(|&num| num & mask == to_keep << i)
            .collect();
        if numbers.len() == 1 {
            break;
        }
    }
    numbers[0]
}
