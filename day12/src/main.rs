use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    time::Instant,
};

const INPUT: [u8; 7461] = *include_bytes!("../input.txt");

fn main() {
    let start_time = Instant::now();

    let inputs = parse_input(&INPUT);
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

fn parse_input<const N: usize>(input: &[u8; N]) -> Grid<N> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut i = 0;
    let mut j = 0;
    let mut data = [0; N];
    let mut k = 0;

    for b in input {
        match b {
            b'\n' => {
                i += 1;
                j = 0;
                continue;
            }
            b'S' => {
                start = (i, j);
            }
            b'E' => {
                end = (i, j);
                data[k] = 25;
            }
            b => {
                data[k] = (b - b'a') as i8;
            }
        }

        j += 1;
        k += 1;
    }

    let heightmap = Array2d {
        data,
        rows: i + 1,
        cols: j,
    };

    Grid {
        heightmap,
        start,
        end,
    }
}

fn part1<const N: usize>(grid: &Grid<N>) -> usize {
    let (n, m) = (grid.heightmap.rows, grid.heightmap.cols);

    let mut queue = VecDeque::from([(0, grid.start.0, grid.start.1)]);
    let mut visited = Array2d {
        data: [false; N],
        rows: n,
        cols: m,
    };

    let end = grid.end;
    visited[grid.start] = true;
    let grid = &grid.heightmap;

    const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((d, i, j)) = queue.pop_front() {
        let h = grid[(i, j)];
        for (oi, oj) in NEIGHBORS {
            let di = (i as isize + oi) as usize;
            let dj = (j as isize + oj) as usize;
            let p = (di, dj);

            if di < n && dj < m && !visited[p] && grid[p] - h <= 1 {
                if p == end {
                    return d + 1;
                }

                visited[p] = true;
                queue.push_back((d + 1, di, dj));
            }
        }
    }

    unreachable!("No path found")
}

fn part2<const N: usize>(grid: &Grid<N>) -> usize {
    let (n, m) = (grid.heightmap.rows, grid.heightmap.cols);

    let mut queue = VecDeque::from([(0, grid.end.0, grid.end.1)]);
    let mut visited = Array2d {
        data: [false; N],
        rows: n,
        cols: m,
    };

    visited[grid.end] = true;
    let grid = &grid.heightmap;

    const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((d, i, j)) = queue.pop_front() {
        let h = grid[(i, j)];
        for (oi, oj) in NEIGHBORS {
            let di = (i as isize + oi) as usize;
            let dj = (j as isize + oj) as usize;
            let p = (di, dj);

            if di < n && dj < m && !visited[p] && h - grid[p] <= 1 {
                if grid[p] == 0 {
                    return d + 1;
                }

                visited[p] = true;
                queue.push_back((d + 1, di, dj));
            }
        }
    }

    unreachable!("No path found")
}

struct Array2d<T, const N: usize> {
    data: [T; N],
    rows: usize,
    cols: usize,
}

struct Grid<const N: usize> {
    heightmap: Array2d<i8, N>,
    start: (usize, usize),
    end: (usize, usize),
}

impl<T: Copy, const N: usize> Index<(usize, usize)> for Array2d<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i * self.cols + j]
    }
}

impl<T: Copy, const N: usize> IndexMut<(usize, usize)> for Array2d<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[i * self.cols + j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [u8; 44] = *b"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(&TEST_INPUT)), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(&TEST_INPUT)), 29);
    }
}
