use std::time::Instant;

const INPUT: &[u8] = include_bytes!("../input.txt");

fn main() {
    let start_time = Instant::now();
    let a = part1();

    let b = part2();
    let end_time = Instant::now();

    println!("Part 1:");
    println!("Start: {} {:?}", a, String::from_utf8_lossy(&INPUT[a - 4..a]));
    println!("\nPart 2:");
    println!("Start: {} {:?}", b, String::from_utf8_lossy(&INPUT[b - 14..b]));
    println!("\nTime: {:?}", end_time - start_time);
}

#[inline]
fn part1() -> usize {
    find_start_marker::<4>()
}

#[inline]
fn part2() -> usize {
    find_start_marker::<14>()
}

#[inline]
fn find_start_marker<const N: usize>() -> usize {
    let mut cardinality = [0; 26];
    for &b in &INPUT[..N] {
        cardinality[(b - b'a') as usize] += 1;
    }

    for i in N..INPUT.len() {
        if cardinality.iter().all(|&i| i <= 1) {
            return i; // It asks for *character count* not index!
        }

        cardinality[(INPUT[i - N] - b'a') as usize] -= 1;
        cardinality[(INPUT[i] - b'a') as usize] += 1;
    }

    unreachable!("Failed to find start marker!");
}
