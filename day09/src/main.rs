use std::{collections::HashSet, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start_time = Instant::now();
    let inputs = parse_input();
    let a = part1(&inputs);

    let b = part2(&inputs);
    let end_time = Instant::now();

    println!("Part 1:");
    println!("Visited: {}", a);
    println!("\nPart 2:");
    println!("Visited: {}", b);
    println!("\nTime: {:?}", end_time - start_time);
}

fn part1(inputs: &[(Move, i32)]) -> usize {
    let mut visited = HashSet::new();

    let mut head = Tail(0, 0);
    let mut tail = head;
    visited.insert(tail);

    for &(dir, dist) in inputs.iter() {
        head.move_head(dir, dist);
        while tail.move_tail(head) {
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(inputs: &[(Move, i32)]) -> usize {
    const N: usize = 9;

    let mut visited = HashSet::new();
    let mut head = Tail(0, 0);
    let mut games = [Tail(0, 0); N];

    for &(dir, dist) in inputs.iter() {
        head.move_head(dir, dist);
        while games[0].move_tail(head) {
            for i in 1..games.len() {
                let dest = games[i - 1];
                games[i].move_tail(dest);
            }

            visited.insert(games[N - 1]);
        }
    }

    visited.len()
}

fn parse_input() -> Vec<(Move, i32)> {
    INPUT
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(dir, dist)| {
            let dir = match dir {
                "U" => Move::Up,
                "D" => Move::Down,
                "L" => Move::Left,
                "R" => Move::Right,
                _ => unreachable!("Invalid direction"),
            };
            let dist = dist.parse().unwrap();
            (dir, dist)
        })
        .collect()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Tail(i32, i32);

#[derive(Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Tail {
    fn move_head(&mut self, m: Move, dist: i32) {
        match m {
            Move::Down => self.1 -= dist,
            Move::Up => self.1 += dist,
            Move::Left => self.0 -= dist,
            Move::Right => self.0 += dist,
        }
    }

    fn move_tail(&mut self, head: Tail) -> bool {
        let (x, y) = (head.0 - self.0, head.1 - self.1);
        if x.abs() < 2 && y.abs() < 2 {
            return false;
        }

        self.0 += x.clamp(-1, 1);
        self.1 += y.clamp(-1, 1);

        true
    }
}
