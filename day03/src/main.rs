use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    println!("====================");
    println!("Part 1:");
    part1(&file);
    println!();
    println!("====================");
    println!("Part 2:");
    part2(&file);
}

fn part1(input: &str) {
    let mut freq = [0; 52];
    for line in input.lines() {
        // Parse into left and right bitset of priorities
        let (left, right) = parse_compartments(line);
        // Find common item by bitwise math
        let common = left & right;
        // Decode it back into the priority value
        let priority = 64 - common.leading_zeros();
        freq[priority as usize - 1] += 1;
    }

    println!(
        "Sum: {}",
        freq.into_iter()
            .enumerate()
            .filter(|(_, f)| f > &0)
            .map(|(i, f)| f * (i as u32 + 1))
            .sum::<u32>()
    );
}

fn part2(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;
    let mut i = 0;
    while i < lines.len() {
        // Convert the group of elves into group of priorities
        let group: Vec<_> = lines[i..i + 3].iter().map(|it| decode(it)).collect();
        // Find common item by bitwise math
        let common = group[0] & group[1] & group[2];
        // Decode it back into the priority value
        sum += 64 - common.leading_zeros();
        i += 3;
    }

    println!("Sum: {}", sum);
}

fn parse_compartments(line: &str) -> (u64, u64) {
    let (left, right) = line.split_at(line.len() >> 1);

    let left = decode(left);
    let right = decode(right);

    (left, right)
}

/// Turns the items into a bitset of priorities
/// We can discard the occurrence count, since we only care about the priority existence
fn decode(items: &str) -> u64 {
    let mut set: u64 = 0;
    for &b in items.as_bytes() {
        let offset = if b >= b'a' { b - b'a' } else { b - b'A' + 26 };

        set |= 1 << offset;
    }

    set
}
