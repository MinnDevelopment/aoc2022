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
        .filter_map(parse_pair)
        .filter(|(a, b, c, d)| (a - c) * (b - d) <= 0)
        .count();

    println!("Count: {}", count);
}

fn part2() {
    let count: usize = INPUT
        .lines()
        .filter_map(parse_pair)
        .filter(|(a, b, c, d)| a.max(c) <= b.min(d))
        .count();

    println!("Count: {}", count);
}

#[inline]
fn parse_pair(line: &str) -> Option<Pair> {
    let (left, right) = line.split_once(',')?;
    let (left_s, left_e) = left.split_once('-')?;
    let (right_s, right_e) = right.split_once('-')?;

    Some((
        left_s.parse().ok()?,
        left_e.parse().ok()?,
        right_s.parse().ok()?,
        right_e.parse().ok()?,
    ))
}

type Pair = (i32, i32, i32, i32);
