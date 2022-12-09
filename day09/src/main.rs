use std::time::Instant;

use fxhash::FxHashSet;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start_time = Instant::now();

    let inputs = parse_input();
    let input_time = Instant::now();

    let a = part1(&inputs);
    let part1_time = Instant::now();

    let b = part2(&inputs);
    let part2_time = Instant::now();

    println!("Input Parsing: {:?}", input_time - start_time);

    println!("\nPart 1:");
    println!("Visited: {}", a);
    println!("Time: {:?}", part1_time - input_time);

    println!("\nPart 2:");
    println!("Visited: {}", b);
    println!("Time: {:?}", part2_time - part1_time);

    println!("\nTotal Time: {:?}", part2_time - start_time);
}

fn part1(inputs: &[(i32, i32)]) -> usize {
    let mut visited = FxHashSet::default();

    let mut head = Knot(0, 0);
    let mut tail = head;
    visited.insert(tail);

    for &motion in inputs {
        head.pull(motion);
        while tail.follow(head) {
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(inputs: &[(i32, i32)]) -> usize {
    const N: usize = 9;

    let mut visited = FxHashSet::default();
    let mut head = Knot(0, 0);
    let mut rope = [Knot(0, 0); N];

    for &motion in inputs {
        head.pull(motion);
        while rope[0].follow(head) {
            for i in 1..N {
                let dest = rope[i - 1];
                rope[i].follow(dest);
            }

            visited.insert(rope[N - 1]);
        }
    }

    visited.len()
}

fn parse_input() -> Box<[(i32, i32)]> {
    INPUT
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(dir, dist)| {
            let d: i32 = dist.parse().unwrap();
            match dir {
                "U" => (0, d),
                "D" => (0, -d),
                "L" => (-d, 0),
                "R" => (d, 0),
                _ => unreachable!("Invalid direction"),
            }
        })
        .collect()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Knot(i32, i32);

impl Knot {
    #[inline]
    fn pull(&mut self, to: (i32, i32)) {
        self.0 += to.0;
        self.1 += to.1;
    }

    #[inline]
    fn follow(&mut self, head: Knot) -> bool {
        let (x, y) = (head.0 - self.0, head.1 - self.1);
        if x.abs() < 2 && y.abs() < 2 {
            return false;
        }

        self.0 += x.clamp(-1, 1);
        self.1 += y.clamp(-1, 1);

        true
    }
}
