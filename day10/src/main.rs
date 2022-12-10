use std::time::Instant;

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
    println!("Sum: {}", a);
    println!("Time: {:?}", part1_time - input_time);

    println!("\nPart 2:");
    println!("CRT Output:\n{}", b);
    println!("Time: {:?}", part2_time - part1_time);

    println!("\nTotal Time: {:?}", part2_time - start_time);
}

fn part1(inputs: &[Instruction]) -> i32 {
    let mut cycle = 0;
    let mut sum = 0;
    let mut x = 1;

    for input in inputs {
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            sum += x * cycle;
        }

        if let Instruction::Addx(v) = input {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                sum += x * cycle;
            }
            x += v;
        }
    }

    sum
}

fn part2(inputs: &[Instruction]) -> String {
    let mut x = 1;

    let mut pos: i32 = 0;
    let mut crt = String::new();

    for input in inputs {
        match (x - pos).abs() {
            0 | 1 => crt.push('#'),
            _ => crt.push('.'),
        }

        pos += 1;

        if pos == 40 {
            crt.push('\n');
            pos = 0;
        }

        if let Instruction::Addx(v) = input {
            match (x - pos).abs() {
                0 | 1 => crt.push('#'),
                _ => crt.push('.'),
            }

            pos += 1;

            if pos == 40 {
                crt.push('\n');
                pos = 0;
            }

            x += v;
        }
    }

    crt
}

fn parse_input() -> Vec<Instruction> {
    INPUT
        .lines()
        .map(|line| match line.split_once(' ') {
            Some(("addx", x)) => Instruction::Addx(x.parse().unwrap()),
            _ => Instruction::Noop,
        })
        .collect()
}

enum Instruction {
    Noop,
    Addx(i32),
}
