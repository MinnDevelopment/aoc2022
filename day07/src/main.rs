use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start_time = Instant::now();
    let shell = parse_input();
    let a = part1(&shell);

    let b = part2(&shell);
    let end_time = Instant::now();

    println!("Part 1:");
    println!("Total: {}", a);
    println!("\nPart 2:");
    println!("Total: {}", b);
    println!("\nTime: {:?}", end_time - start_time);
}

#[inline]
fn part1(shell: &Shell) -> usize {
    shell
        .directories
        .values()
        .copied()
        .filter(|&size| size <= 100000)
        .sum()
}

#[inline]
fn part2(shell: &Shell) -> usize {
    let unused = 70000000 - shell.directories[""];
    let required = 30000000 - unused;

    shell
        .directories
        .values()
        .copied()
        .filter(|&size| size >= required)
        .min()
        .unwrap()
}

fn parse_input() -> Shell {
    let mut shell = Shell::new();
    for (i, line) in INPUT.lines().enumerate() {
        if line.is_empty() {
            break;
        }

        if let Some(dir) = line.strip_prefix("$ cd ") {
            shell.execute_command(Command::Cd(dir.to_string()));
        } else if line.starts_with("$ ls") {
            let data = INPUT
                .lines()
                .skip(i + 1)
                .take_while(|line| !line.starts_with('$') && !line.is_empty())
                .map(String::from)
                .collect();
            shell.execute_command(Command::Ls(data));
        }
    }

    shell
}

#[derive(Clone)]
struct Path {
    segments: Vec<String>,
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Self {
            segments: s[1..].split('/').map(|s| s.to_string()).collect(),
        }
    }
}

impl Path {
    fn parent(&self) -> Option<Self> {
        if self.segments.len() == 1 {
            None
        } else {
            let mut segments = self.segments.clone();
            segments.pop();
            Some(Self { segments })
        }
    }

    fn get_relative(&self, path: &str) -> Self {
        let mut segments = self.segments.clone();
        if let Some(path) = path.strip_prefix('/') {
            for segment in path.split('/').skip(segments.len()) {
                segments.push(segment.to_string());
            }
        } else {
            for segment in path.split('/') {
                match segment {
                    ".." => {
                        segments.pop();
                    }
                    _ => segments.push(segment.to_string()),
                }
            }
        }
        Self { segments }
    }

    fn string(&self) -> String {
        self.segments.join("/")
    }
}

struct Shell {
    current_dir: Path,
    directories: HashMap<String, usize>,
}

enum Command {
    Cd(String),
    Ls(Vec<String>),
}

impl Shell {
    fn new() -> Self {
        let mut directories = HashMap::new();
        directories.insert("".to_string(), 0);
        Self {
            current_dir: Path { segments: vec![] },
            directories,
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Cd(dir) => {
                self.current_dir = self.current_dir.get_relative(&dir);
                self.directories
                    .entry(self.current_dir.string())
                    .or_insert(0);
            }
            Command::Ls(output) => {
                let mut total = 0;
                for line in output {
                    let (size, _) = line.split_once(' ').unwrap();
                    if let Ok(size) = size.parse::<usize>() {
                        total += size;
                    }
                }

                let mut path = Some(self.current_dir.clone());
                while let Some(p) = path {
                    self.directories.entry(p.string()).and_modify(|size| {
                        *size += total;
                    });
                    path = p.parent();
                }
            }
        }
    }
}
