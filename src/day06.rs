use std::{mem, ops::Deref, str::FromStr};

use crate::utils::{print_day_header, print_result, print_section, read_input};

const EXAMPLE: &str = "
123 328  51 64\x20
 45 64  387 23\x20
  6 98  215 314
*   +   *   +
";

#[derive(Debug, Copy, Clone, PartialEq)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(format!("Invalid operation: {}", s)),
        }
    }
}

impl Op {
    fn apply(&self, numbers: &[u64]) -> u64 {
        match self {
            Op::Add => numbers.iter().sum(),
            Op::Mul => numbers.iter().product(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Problem {
    numbers: Vec<u64>,
    op: Op,
}

impl Problem {
    fn solve(&self) -> u64 {
        self.op.apply(&self.numbers)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Problems {
    problems: Vec<Problem>,
}

impl Deref for Problems {
    type Target = [Problem];

    fn deref(&self) -> &Self::Target {
        &self.problems
    }
}

impl Problems {
    fn total(&self) -> u64 {
        self.iter().map(|p| p.solve()).sum()
    }
}

fn parse_operations(lines: &[&str]) -> Vec<Op> {
    lines
        .last()
        .expect("No operations line found")
        .split_whitespace()
        .map(|raw_op| raw_op.parse().expect("Invalid operation"))
        .collect()
}

fn parse01(raw_data: &str) -> Problems {
    let lines: Vec<_> = raw_data.trim().lines().collect();
    let operations: Vec<_> = parse_operations(&lines);

    let all_problems_numbers: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|&row| {
            row.split_whitespace()
                .map(|num_str| num_str.parse().expect("Invalid number"))
                .collect()
        })
        .collect();

    let problems = operations
        .into_iter()
        .enumerate()
        .map(|(i, op)| Problem {
            op,
            numbers: all_problems_numbers.iter().map(|nums| nums[i]).collect(),
        })
        .collect();

    Problems { problems }
}

fn parse_column(row_chars: &[Vec<char>], col_idx: usize) -> Option<u64> {
    let vertical: String = row_chars
        .iter()
        .map(|chars| chars.get(col_idx).copied().unwrap_or(' '))
        .collect();
    vertical.trim().parse().ok()
}

fn parse02(raw_data: &str) -> Problems {
    let lines: Vec<_> = raw_data.trim().lines().collect();
    let operations = parse_operations(&lines);
    let number_rows: Vec<_> = lines.iter().take(lines.len() - 1).copied().collect();

    // Use the maximum line length to avoid missing columns
    let max_width = number_rows.iter().map(|line| line.len()).max().unwrap_or(0);
    let row_chars: Vec<Vec<char>> = number_rows
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let mut problem_groups: Vec<Vec<u64>> = Vec::with_capacity(operations.len());
    let mut current_group: Vec<u64> = vec![];

    for col_idx in 0..max_width {
        match parse_column(&row_chars, col_idx) {
            Some(num) => current_group.push(num),
            None if !current_group.is_empty() => {
                // Hit the separator, flush the group
                problem_groups.push(mem::take(&mut current_group));
            }
            None => {}
        }
    }

    // Last group
    if !current_group.is_empty() {
        problem_groups.push(current_group);
    }

    let problems: Vec<Problem> = problem_groups
        .into_iter()
        .zip(operations)
        .map(|(numbers, op)| Problem { numbers, op })
        .collect();

    Problems { problems }
}

fn part01(problems: &Problems) -> u64 {
    problems.total()
}

fn part02(problems: &Problems) -> u64 {
    problems.total()
}

pub fn day06() {
    print_day_header(6);

    let example_data_01 = parse01(EXAMPLE);
    let actual_data_01 = parse01(&read_input(6));

    let example_data_02 = parse02(EXAMPLE);
    let actual_data_02 = parse02(&read_input(6));

    // dbg!(actual_data_02);

    print_section("Part 1 (Example)");
    print_result("The total number is: ", part01(&example_data_01));

    print_section("Part 1 (Actual)");
    print_result("The total number is: ", part01(&actual_data_01));

    print_section("Part 2 (Example)");
    print_result("The total number is: ", part02(&example_data_02));

    print_section("Part 2 (Actual)");
    print_result("The total number is: ", part02(&actual_data_02));
}
