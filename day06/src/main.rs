use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    part1();

    println!("\nPart 2:");
    part2();
}

fn part1() {
    let start = find_start_marker::<4>();
    println!("Start: {} {:?}", start, &INPUT[start - 4..start]);
}

fn part2() {
    let start = find_start_marker::<14>();
    println!("Start: {} {:?}", start, &INPUT[start - 14..start]);
}

fn find_start_marker<const N: usize>() -> usize {
    let mut window: VecDeque<_> = INPUT.bytes().take(N).collect();
    let mut cardinality = [0; 26];
    for &b in window.iter() {
        cardinality[(b - b'a') as usize] += 1;
    }

    for (i, b) in INPUT.bytes().enumerate().skip(N) {
        if cardinality.iter().all(|&i| i <= 1) {
            return i; // It asks for *character count* not index!
        }

        let old = window.pop_front().unwrap();
        cardinality[(old - b'a') as usize] -= 1;

        window.push_back(b);
        cardinality[(b - b'a') as usize] += 1;
    }

    panic!("Failed to find start marker!");
}
