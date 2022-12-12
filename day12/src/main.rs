use std::{collections::VecDeque, time::Instant};

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
    println!("Steps: {}", a);
    println!("Time: {:?}", part1_time - input_time);

    println!("\nPart 2:");
    println!("Steps: {}", b);
    println!("Time: {:?}", part2_time - part1_time);

    println!("\nTotal Time: {:?}", part2_time - start_time);
}

fn parse_input(input: &str) -> Grid {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut heightmap: Box<[Box<[i8]>]> = input
        .lines()
        .map(|line| line.bytes().map(|b| b as i8).collect())
        .collect();

    for i in 0..heightmap.len() {
        for j in 0..heightmap[i].len() {
            heightmap[i][j] = match heightmap[i][j] as u8 {
                b'S' => {
                    start = (i, j);
                    0
                }
                b'E' => {
                    end = (i, j);
                    25
                }
                b => (b - b'a') as i8,
            };
        }
    }

    Grid {
        heightmap,
        start,
        end,
    }
}

fn part1(grid: &Grid) -> usize {
    let (n, m) = (grid.heightmap.len(), grid.heightmap[0].len());

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];

    visited[grid.start.0][grid.start.1] = true;
    queue.push_back((0, grid.start.0, grid.start.1));

    const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((d, i, j)) = queue.pop_front() {
        let h = grid.heightmap[i][j];
        for (oi, oj) in NEIGHBORS {
            let di = (i as isize + oi) as usize;
            let dj = (j as isize + oj) as usize;

            if di < n && dj < m && !visited[di][dj] && grid.heightmap[di][dj] - h <= 1 {
                visited[di][dj] = true;
                if (di, dj) == grid.end {
                    return d + 1;
                }

                queue.push_back((d + 1, di, dj));
            }
        }
    }

    panic!("No path found")
}

fn part2(grid: &Grid) -> usize {
    let (n, m) = (grid.heightmap.len(), grid.heightmap[0].len());

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];

    visited[grid.end.0][grid.end.1] = true;
    queue.push_back((0, grid.end.0, grid.end.1));

    const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((d, i, j)) = queue.pop_front() {
        let h = grid.heightmap[i][j];
        for (oi, oj) in NEIGHBORS {
            let di = (i as isize + oi) as usize;
            let dj = (j as isize + oj) as usize;

            if di < n && dj < m && !visited[di][dj] && h - grid.heightmap[di][dj] <= 1 {
                visited[di][dj] = true;
                if grid.heightmap[di][dj] == 0 {
                    return d + 1;
                }

                queue.push_back((d + 1, di, dj));
            }
        }
    }

    panic!("No path found")
}

struct Grid {
    heightmap: Box<[Box<[i8]>]>,
    start: (usize, usize),
    end: (usize, usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 29);
    }
}
