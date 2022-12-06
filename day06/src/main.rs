use std::collections::{VecDeque, HashMap};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    part1();

    println!("\nPart 2:");
    part2();
}

fn part1() {
    let start = find_start_marker(4);
    println!("Start: {} {:?}", start, &INPUT[start - 4..start]);
}

fn part2() {
    let start = find_start_marker(14);
    println!("Start: {} {:?}", start, &INPUT[start - 14..start]);
}

fn find_start_marker(len: usize) -> usize {
    let mut window: VecDeque<_> = INPUT.bytes().take(len).collect();
    let mut cardinality = HashMap::with_capacity(len);
    for &b in window.iter() {
        *cardinality.entry(b).or_insert(0) += 1;
    }

    if cardinality.len() == len {
        return len;
    }

    for (i, b) in INPUT.bytes().enumerate().skip(len) {
        let old = window.pop_front().unwrap();
        if let Some(count) = cardinality.remove(&old) {
            if count > 1 {
                cardinality.insert(old, count - 1);
            }
        }
        
        window.push_back(b);
        *cardinality.entry(b).or_insert(0) += 1;

        if cardinality.len() == len {
            return i + 1; // It asks for *character count* not index!
        }
    }

    panic!("Failed to find start marker!");
}
