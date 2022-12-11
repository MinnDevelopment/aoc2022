use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start_time = Instant::now();

    let inputs = parse_input(INPUT);
    let input_time = Instant::now();

    let a = part1(&inputs);
    let part1_time = Instant::now();

    let b = part2(&inputs);
    let part2_time = Instant::now();

    println!("Input Parsing: {:?}", input_time - start_time);

    println!("\nPart 1:");
    println!("Monkey Business: {}", a);
    println!("Time: {:?}", part1_time - input_time);

    println!("\nPart 2:");
    println!("Monkey Business: {}", b);
    println!("Time: {:?}", part2_time - part1_time);

    println!("\nTotal Time: {:?}", part2_time - start_time);
}

fn parse_input(input: &str) -> Vec<Monkey> {
    macro_rules! parse_num {
        ($line:expr) => {
            $line.rsplit_once(' ').unwrap().1.parse().unwrap()
        };
    }

    let lines: Vec<_> = input.lines().collect();
    let mut i = 0;
    let mut monkeys = Vec::new();
    while i < lines.len() {
        if lines[i].starts_with("Monkey") {
            let items: Vec<usize> = lines[i + 1]
                .split([',', ' '])
                .filter_map(|n| n.parse().ok())
                .collect();

            let op = match lines[i + 2].rsplitn(3, ' ').collect::<Vec<_>>().as_slice() {
                ["old", "*", ..] => Operation::Squared,
                [n, "*", ..] => Operation::Multiply(n.parse().unwrap()),
                [n, "+", ..] => Operation::Add(n.parse().unwrap()),
                _ => unreachable!("Invalid operation"),
            };

            let divisor = parse_num!(lines[i + 3]);
            let on_true = parse_num!(lines[i + 4]);
            let on_false = parse_num!(lines[i + 5]);
            monkeys.push(Monkey {
                items,
                op,
                divisor,
                on_true,
                on_false,
            });

            i += 6;
        }

        i += 1;
    }

    monkeys
}

fn part1(brains: &[Monkey]) -> usize {
    let mut monkeys: Vec<_> = brains.iter().map(|m| m.items.clone()).collect();
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let items: Vec<_> = monkeys[i]
                .drain(..)
                .map(|item| brains[i].op.apply(item) / 3)
                .collect();
            inspections[i] += items.len();
            for item in items {
                if item % brains[i].divisor == 0 {
                    monkeys[brains[i].on_true].push(item);
                } else {
                    monkeys[brains[i].on_false].push(item);
                }
            }
        }
    }

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn part2(brains: &[Monkey]) -> usize {
    let mut monkeys: Vec<_> = brains.iter().map(|m| m.items.clone()).collect();
    let mut inspections = vec![0; monkeys.len()];

    let bound: usize = brains.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items: Vec<_> = monkeys[i]
                .drain(..)
                .map(|item| brains[i].op.apply(item) % bound)
                .collect();
            inspections[i] += items.len();
            for item in items {
                if item % brains[i].divisor == 0 {
                    monkeys[brains[i].on_true].push(item);
                } else {
                    monkeys[brains[i].on_false].push(item);
                }
            }
        }
    }

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    divisor: usize,
    on_true: usize,
    on_false: usize,
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Squared,
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Add(n) => item + n,
            Operation::Multiply(n) => item * n,
            Operation::Squared => item * item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2713310158);
    }
}
