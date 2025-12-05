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

const ADJACENT_DELTAS: [(i16, i16); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const MAX_NEIGHBORS_TO_BE_CLEANABLE: u8 = 4;

type Matrix = Vec<Vec<Space>>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Paper,
    Empty,
}

fn parse(raw_data: &str) -> Matrix {
    raw_data
        .trim()
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '@' => Space::Paper,
                    '.' => Space::Empty,
                    _ => panic!("Invalid character in input: '{}'", ch),
                })
                .collect()
        })
        .collect()
}

fn count_paper_neighbours(matrix: &Matrix, row: usize, col: usize) -> u8 {
    let rows = matrix.len() as i16;
    let cols = matrix[0].len() as i16;

    ADJACENT_DELTAS
        .iter()
        .filter_map(|(di, dj)| {
            let i2 = row as i16 + di;
            let j2 = col as i16 + dj;

            match (i2, j2) {
                (i2, _) if i2 < 0 || i2 >= rows as i16 => None,
                (_, j2) if j2 < 0 || j2 >= cols as i16 => None,
                (i2, j2) => Some(matrix[i2 as usize][j2 as usize]),
            }
        })
        .filter(|&space| space == Space::Paper)
        .count() as u8
}

fn clean_warehouse(matrix: &Matrix) -> (Matrix, u16) {
    let mut clean_matrix = matrix.clone();
    let mut n_cleaned  = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == Space::Paper {
                let paper_neighbours = count_paper_neighbours(matrix, i, j);

                if paper_neighbours < MAX_NEIGHBORS_TO_BE_CLEANABLE {
                    clean_matrix[i][j] = Space::Empty;
                    n_cleaned += 1
                }
            }
        }
    }

    (clean_matrix, n_cleaned)
}

fn part01(matrix: &Matrix) -> u16 {
    clean_warehouse(matrix).1
}

fn part02(data: &Matrix) -> u16 {
    let mut current_matrix = data.clone();
    let mut total_removed = 0;

    loop {
        let (new_matrix, n_removed) = clean_warehouse(&current_matrix);
        if n_removed == 0 {
            break;
        }

        total_removed += n_removed;
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
