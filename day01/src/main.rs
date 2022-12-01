use std::{
    cmp::Reverse,
    fs,
};

use anyhow::Result;

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;

    println!("====================");
    println!("Part 1:");
    part1(&file);
    println!();
    println!("====================");
    println!("Part 2:");
    part2(&file);

    Ok(())
}

fn part1(input: &str) {
    let mut total: u64 = 0;
    let mut max = 0;

    for line in input.lines() {
        match line.parse::<u64>() {
            Ok(n) => {
                total += n;
                if total > max {
                    max = total;
                }
            }
            _ => {
                total = 0;
            }
        }
    }

    println!("Max: {}", max);
}

fn part2(input: &str) {
    let mut elves = Vec::new();

    elves.push(Reverse(0));

    for line in input.lines() {
        match line.parse::<u64>() {
            Ok(n) => {
                elves.last_mut().unwrap().0 += n;
            }
            _ => {
                elves.push(Reverse(0));
            }
        }
    }

    elves.sort();

    let mut total = 0;
    for (i, &v) in elves[0..3].iter().enumerate() {
        println!("{}. {}", i+1, v.0);
        total += v.0;
    }

    println!("Total: {}", total);
}
