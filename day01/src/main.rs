use std::fs;

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
    let mut total = 0;
    let mut max = 0;

    for line in input.lines() {
        match line.parse::<u32>() {
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
    let mut top3 = [0; 3];

    let mut total = 0;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => {
                total += n;
            }
            _ => {
                replace_min(&mut top3, total);
                total = 0;
            }
        }
    }

    replace_min(&mut top3, total);

    top3.sort();

    let mut total = 0;
    for (i, v) in top3.into_iter().enumerate() {
        println!("{}. {}", i + 1, v);
        total += v;
    }

    println!("Total: {}", total);
}

#[inline(always)]
fn replace_min(arr: &mut [u32; 3], v: u32) {
    let (min, min_id) = arr.iter().enumerate().map(|(i, &n)| (n, i)).min().unwrap();

    if v > min {
        arr[min_id] = v;
    }
}
