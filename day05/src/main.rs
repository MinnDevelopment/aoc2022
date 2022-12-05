const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part 1:");
    part1();

    println!("\nPart 2:");
    part2();
}

fn part1() {
    let mut cargo = CargoStacks::parse(INPUT);

    let moves = INPUT
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(parse_move);

    for (num, src, dst) in moves {
        for _ in 0..num {
            cargo.make_move(src, dst);
        }
    }

    println!("Top: {}", cargo.top());
}

fn part2() {
    let mut cargo = CargoStacks::parse(INPUT);

    let moves = INPUT
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(parse_move);

    for (num, src, dst) in moves {
        cargo.make_multi_move(num, src, dst);
    }

    println!("Top: {}", cargo.top());
}

struct CargoStacks {
    stacks: Vec<Vec<u8>>,
}

// "move 1 from 3 to 5"
fn parse_move(line: &str) -> (usize, usize, usize) {
    let parts: Vec<usize> = line
        .split(' ')
        .skip(1)
        .step_by(2)
        .filter_map(|part| part.parse().ok())
        .collect();
    (parts[0], parts[1] - 1, parts[2] - 1)
}

impl CargoStacks {
    fn top(&self) -> String {
        let mut top = String::new();
        for stack in &self.stacks {
            match stack.last() {
                Some(&b) if b >= b'A' && b <= b'Z' => top.push(b as char),
                _ => top.push(' '),
            }
        }
        top
    }

    fn make_move(&mut self, src: usize, dest: usize) {
        if let Some(cargo) = self.stacks[src].pop() {
            self.stacks[dest].push(cargo);
        }
    }

    fn make_multi_move(&mut self, n: usize, src: usize, dest: usize) {
        let src = &mut self.stacks[src];
        let mut cargo: Vec<_> = src.drain(src.len() - n..).collect();
        self.stacks[dest].append(&mut cargo);
    }

    fn parse(input: &str) -> Self {
        let mut stacks = vec![vec![]; 9];
        for line in input.lines() {
            if line.starts_with(" 1") {
                break;
            }

            line[1..]
                .bytes()
                .step_by(4)
                .enumerate()
                .filter(|(_, b)| b != &b' ')
                .for_each(|(s, item)| stacks[s].push(item));
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        Self { stacks }
    }
}
