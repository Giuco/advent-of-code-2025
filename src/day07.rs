use crate::utils::{print_day_header, print_result, print_section, read_input};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const EXAMPLE: &str = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

type Pos = (usize, usize);

#[derive(Debug)]
enum Space {
    Empty,
    Splitter,
}

impl FromStr for Space {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" | "." => Ok(Space::Empty),
            "^" => Ok(Space::Splitter),
            _ => Err(format!("Invalid space: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Matrix {
    matrix: Vec<Vec<Space>>,
    start: Pos,
}

impl FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let start_col = s
            .chars()
            .position(|ch| ch == 'S')
            .ok_or("Couldn't find start position")?;

        let matrix: Result<Vec<Vec<Space>>, _> = s
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_string().parse()).collect())
            .collect();

        Ok(Matrix {
            matrix: matrix?,
            start: (0, start_col),
        })
    }
}

impl Matrix {
    fn rows(&self) -> &[Vec<Space>] {
        &self.matrix
    }

    fn height(&self) -> usize {
        self.matrix.len()
    }

    fn width(&self) -> usize {
        self.matrix[0].len()
    }

    fn get(&self, row: usize, col: usize) -> Option<&Space> {
        self.matrix.get(row).and_then(|r| r.get(col))
    }
}

fn parse(raw_data: &str) -> Matrix {
    raw_data.parse().expect("Couldn't parse the matrix")
}

fn part01(matrix: &Matrix) -> usize {
    let mut exploration_queue: Vec<Pos> = vec![matrix.start];
    let mut n_splits_hit: HashSet<Pos> = HashSet::new();

    while let Some((i, j)) = exploration_queue.pop() {
        let (ii, jj) = (i + 1, j);

        if ii >= matrix.height() || jj >= matrix.width() || n_splits_hit.contains(&(ii, jj)) {
            continue;
        }

        let space = matrix.get(ii, jj).expect("Should find it");

        match &space {
            Space::Empty => exploration_queue.push((ii, jj)),
            Space::Splitter => {
                exploration_queue.extend_from_slice(&[(ii, jj - 1), (ii, jj + 1)]);
                n_splits_hit.insert((ii, jj));
            }
        }
    }

    n_splits_hit.len()
}

fn part02(matrix: &Matrix) -> u64 {
    let mut timelines: HashMap<usize, u64> = HashMap::from([(matrix.start.1, 1)]);

    for row in matrix.rows().iter().skip(1) {
        let mut next_timelines: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &timelines {
            match &row[col] {
                Space::Splitter => {
                    if let Some(left) = col.checked_sub(1) {
                        *next_timelines.entry(left).or_default() += count;
                    }
                    if col + 1 < matrix.width() {
                        *next_timelines.entry(col + 1).or_default() += count;
                    }
                }
                Space::Empty => {
                    *next_timelines.entry(col).or_default() += count;
                }
            }
        }

        timelines = next_timelines;
    }

    timelines.values().sum()
}

pub fn day07() {
    print_day_header(7);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(7));

    print_section("Part 1 (Example)");
    print_result("This is the number of splits", part01(&example_data));

    print_section("Part 1 (Actual)");
    print_result("This is the number of splits", part01(&actual_data));

    print_section("Part 2 (Example)");
    print_result("This is the number of splits", part02(&example_data));

    print_section("Part 2 (Actual)");
    print_result("This is the number of splits", part02(&actual_data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part01(&matrix), 21)
    }

    #[test]
    fn test_part02_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part02(&matrix), 40)
    }
}
