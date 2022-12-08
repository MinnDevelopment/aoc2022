use std::{collections::HashSet, time::Instant};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let start_time = Instant::now();
    let grid = Grid::parse_input();
    let a = part1(&grid);

    let b = part2(&grid);
    let end_time = Instant::now();

    println!("Part 1:");
    println!("Total: {}", a);
    println!("\nPart 2:");
    println!("Highest: {}", b);
    println!("\nTime: {:?}", end_time - start_time);
}

fn part1(grid: &Grid) -> usize {
    let mut seen = HashSet::new();

    // Check for trees visible from north
    for j in 1..grid.num_cols - 1 {
        let mut height = grid.data[0][j];
        for i in 1..grid.num_rows - 1 {
            let value = grid.data[i][j];
            if value > height {
                seen.insert((i, j));
                height = value;
            }
        }
    }

    // Check for trees visible from east
    for i in 1..grid.num_rows - 1 {
        let mut height = grid.data[i][grid.num_cols - 1];
        for j in (1..grid.num_cols - 1).rev() {
            let value = grid.data[i][j];
            if value > height {
                seen.insert((i, j));
                height = value;
            }
        }
    }

    // Check for trees visible from south
    for j in 1..grid.num_cols - 1 {
        let mut height = grid.data[grid.num_rows - 1][j];
        for i in (1..grid.num_rows - 1).rev() {
            let value = grid.data[i][j];
            if value > height {
                seen.insert((i, j));
                height = value;
            }
        }
    }

    // Check for trees visible from west
    for i in 1..grid.num_rows - 1 {
        let mut height = grid.data[i][0];
        for j in 1..grid.num_cols - 1 {
            let value = grid.data[i][j];
            if value > height {
                seen.insert((i, j));
                height = value;
            }
        }
    }

    2 * (grid.num_rows + grid.num_cols - 2) + seen.len()
}

fn part2(grid: &Grid) -> usize {
    let mut scores = vec![vec![1; grid.num_cols]; grid.num_rows];

    // Check approach from the north
    for j in 1..grid.num_cols - 1 {
        let mut heights = [0_usize; 10];
        for i in 1..grid.num_rows - 1 {
            let height = grid.data[i][j] as usize;

            // Check how far you can see from this height into the north direction
            let mut min = i - heights[height];
            for h in &heights[height + 1..] {
                min = min.min(i - h);
            }

            scores[i][j] *= min;

            // Store the last position where this height showed up
            heights[height] = i;
        }
    }

    // Check approach from the east
    for i in 1..grid.num_rows - 1 {
        let mut heights = [grid.num_cols - 1; 10];
        for j in (1..grid.num_cols - 1).rev() {
            let height = grid.data[i][j] as usize;

            // Check how far you can see from this height into the east direction
            let mut min = heights[height] - j;
            for h in &heights[height + 1..] {
                min = min.min(h - j);
            }

            scores[i][j] *= min;

            // Store the last position where this height showed up
            heights[height] = j;
        }
    }

    // Check approach from the south
    for j in 1..grid.num_cols - 1 {
        let mut heights = [grid.num_rows - 1; 10];
        for i in (1..grid.num_rows - 1).rev() {
            let height = grid.data[i][j] as usize;

            // Check how far you can see from this height into the south direction
            let mut min = heights[height] - i;
            for h in &heights[height + 1..] {
                min = min.min(h - i);
            }

            scores[i][j] *= min;

            // Store the last position where this height showed up
            heights[height] = i;
        }
    }

    let mut max = 0;

    // Check approach from the west
    for i in 1..grid.num_rows - 1 {
        let mut heights = [0_usize; 10];
        for j in 1..grid.num_cols - 1 {
            let height = grid.data[i][j] as usize;

            // Check how far you can see from this height into the west direction
            let mut min = j - heights[height];
            for h in &heights[height + 1..] {
                min = min.min(j - h);
            }

            scores[i][j] *= min;
            // Keep track of max score to avoid second pass
            max = max.max(scores[i][j]);

            // Store the last position where this height showed up
            heights[height] = j;
        }
    }

    max
}

struct Grid {
    data: Vec<Vec<u8>>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    fn parse_input() -> Grid {
        let data: Vec<Vec<_>> = INPUT
            .lines()
            .map(|line| line.bytes().map(|c| c - b'0').collect())
            .collect();
        Grid {
            num_rows: data.len(),
            num_cols: data[0].len(),
            data,
        }
    }
}
