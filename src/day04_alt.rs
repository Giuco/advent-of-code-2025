/// Slower than the matrix approach, but much more fun!
use std::collections::HashSet;

use crate::utils::{print_day_header, print_result, print_section, read_input};

const EXAMPLE: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

const ADJACENT_DELTAS: [(i32, i32); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const MAX_NEIGHBORS_TO_BE_CLEANABLE: i32 = 4;

type Positions = HashSet<(i32, i32)>;

fn parse(raw_data: &str) -> Positions {
    raw_data
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(j, ch)| (ch == '@').then_some((i as i32, j as i32)))
        })
        .collect()
}

fn count_paper_neighbours(positions: &Positions, i: i32, j: i32) -> i32 {
    ADJACENT_DELTAS
        .iter()
        .filter(|&&(di, dj)| {
            let (ii, jj) = (i + di, j + dj);
            positions.contains(&(ii, jj))
        })
        .count() as i32
}

fn clean_warehouse(positions: &Positions) -> Positions {
    positions
        .iter()
        .filter(|&&(i, j)| {
            let n_neighbours = count_paper_neighbours(positions, i, j);
            n_neighbours >= MAX_NEIGHBORS_TO_BE_CLEANABLE
        })
        .copied()
        .collect()
}

fn part01(positions: &Positions) -> usize {
    positions.len() - clean_warehouse(&positions).len()
}

fn part02(positions: &Positions) -> usize {
    let mut current_matrix = positions.clone();
    let mut total_removed = 0;

    loop {
        let new_matrix = clean_warehouse(&current_matrix);
        let n_cleaned = current_matrix.len() - new_matrix.len();

        if n_cleaned == 0 {
            break;
        }

        total_removed += n_cleaned;
        current_matrix = new_matrix;
    }

    total_removed
}

pub fn day04() {
    print_day_header(4);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(4));

    print_section("Part 1 (Example)");
    print_result(
        "This is the number of rolls that can be accessed by the forklift",
        part01(&example_data),
    );

    print_section("Part 1 (Actual)");
    print_result(
        "This is the number of rolls that can be accessed by the forklift",
        part01(&actual_data),
    );

    print_section("Part 2 (Example)");
    print_result(
        "This is the number of rolls that can be accessed by the forklift",
        part02(&example_data),
    );

    print_section("Part 2 (Actual)");
    print_result(
        "The dial passed through zero",
        format!("{} times", part02(&actual_data)),
    );
}
