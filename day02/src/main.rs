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

#[repr(u8)]
enum Outcome {
    Loser,
    Winner,
    Draw,
}

impl Outcome {
    const fn value(&self) -> u32 {
        match self {
            Outcome::Loser => 0,
            Outcome::Draw => 3,
            Outcome::Winner => 6,
        }
    }

    const fn from(b: u8) -> Outcome {
        match b {
            b'X' => Outcome::Loser,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Winner,
            _ => unreachable!(),
        }
    }

    const fn choice(&self, a: u8) -> u32 {
        match (self, a) {
            (Outcome::Loser, b'A') => 3, // One lower (modulo)
            (Outcome::Loser, b'B') => 1,
            (Outcome::Loser, b'C') => 2,
            (Outcome::Winner, b'A') => 2, // One higher (modulo)
            (Outcome::Winner, b'B') => 3,
            (Outcome::Winner, b'C') => 1,
            (Outcome::Draw, b'A') => 1, // Equal
            (Outcome::Draw, b'B') => 2,
            (Outcome::Draw, b'C') => 3,
            _ => unreachable!(),
        }
    }
}

/// Determine the winner (b is the player)
const fn outcome(a: u32, b: u32) -> Outcome {
    match (a, b) {
        (1, 2) => Outcome::Winner, // Winner if b is 1 right of a
        (3, 1) => Outcome::Winner,
        (2, 3) => Outcome::Winner,
        (2, 1) => Outcome::Loser,  // Loser if b is 1 left of a
        (1, 3) => Outcome::Loser,
        (3, 2) => Outcome::Loser,
        _ => Outcome::Draw,        // Draw if a and b are equal
    }
}

// Converts A/B/C and X/Y/Z to their values 1/2/3
const fn value(base: u8, value: u8) -> u32 {
    (value - base + 1) as u32
}

fn part1(input: &str) {
    let sum: u32 = input
        .lines()
        .filter(|line| line.len() >= 3)
        .map(|line| {
            // "A B" => (b'A', b'B')
            let b = line.as_bytes();
            (b[0], b[2])
        })
        .map(|(a, b)| {
            // Decode into values
            let a = value(b'A', a);
            let b = value(b'X', b);
            // Make move and sum outcome
            b + outcome(a, b).value()
        })
        .sum();

    println!("Sum: {}", sum);
}

fn part2(input: &str) {
    let sum: u32 = input
        .lines()
        .filter(|line| line.len() >= 3)
        .map(|line| {
            // "A B" => (b'A', b'B')
            let b = line.as_bytes();
            (b[0], b[2])
        })
        .map(|(a, b)| {
            // Pick the recommended strategy
            let outcome = Outcome::from(b);
            // Take the required move and sum the result
            outcome.value() + outcome.choice(a)
        })
        .sum();

    println!("Sum: {}", sum);
}
