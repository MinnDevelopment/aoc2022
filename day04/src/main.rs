use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    part1();

    println!("\n");
    println!("Part 2:");
    part2();
}

fn part1() {
    let count: usize = INPUT
        .lines()
        .filter_map(|line| parse_pair(line))
        .filter(|pair| {
            (pair.first.contains(pair.second.start()) && pair.first.contains(pair.second.end()))
                || (pair.second.contains(pair.first.start())
                    && pair.second.contains(pair.first.end()))
        })
        .count();

    println!("Count: {}", count);
}

fn part2() {
    let count: usize = INPUT
        .lines()
        .filter_map(|line| parse_pair(line))
        .filter(|pair| {
            pair.first.contains(pair.second.start()) || pair.second.contains(pair.first.start())
        })
        .count();

    println!("Count: {}", count);
}

#[inline]
fn parse_pair(line: &str) -> Option<Pair> {
    let (left, right) = line.split_once(",")?;
    let (left_s, left_e) = left.split_once("-")?;
    let (right_s, right_e) = right.split_once("-")?;

    Some(Pair {
        first: left_s.parse().ok()?..=left_e.parse().ok()?,
        second: right_s.parse().ok()?..=right_e.parse().ok()?,
    })
}

struct Pair {
    first: RangeInclusive<i32>,
    second: RangeInclusive<i32>,
}
