use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::utils::{print_day_header, print_result, print_section, read_input};

const EXAMPLE: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, n: u64) -> bool {
        (self.start..=self.end).contains(&n)
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    #[allow(dead_code)] // For AoC, we might not use it
    fn is_empty(&self) -> bool {
        self.end < self.start
    }
}

#[derive(Debug)]
enum ParseRangeError {
    MissingDash,
    InvalidStart(ParseIntError),
    InvalidEnd(ParseIntError),
}

impl Display for ParseRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRangeError::MissingDash => write!(f, "Range must contain a dash"),
            ParseRangeError::InvalidStart(e) => write!(f, "Invalid start value: {}", e),
            ParseRangeError::InvalidEnd(e) => write!(f, "Invalid end value: {}", e),
        }
    }
}

impl std::error::Error for ParseRangeError {}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_start, raw_end) = s.split_once("-").ok_or(ParseRangeError::MissingDash)?;

        Ok(Range {
            start: raw_start.parse().map_err(ParseRangeError::InvalidStart)?,
            end: raw_end.parse().map_err(ParseRangeError::InvalidEnd)?,
        })
    }
}

#[derive(Debug)]
struct Inventory {
    fresh_ingredients: Vec<Range>,
    available_ingredients: Vec<u64>,
}

fn parse(raw_data: &str) -> Inventory {
    let (raw_fresh_ingredients, raw_available_ingredients) =
        raw_data.trim().split_once("\n\n").unwrap();

    let fresh_ingredients = raw_fresh_ingredients
        .lines()
        .map(|row| row.parse().expect("Value is not parseable"))
        .collect();

    let available_ingredients = raw_available_ingredients
        .lines()
        .map(|row| row.parse().expect("Value is not parseable"))
        .collect();

    Inventory {
        fresh_ingredients,
        available_ingredients,
    }
}

fn part01(inventory: &Inventory) -> usize {
    inventory
        .available_ingredients
        .iter()
        .filter(|&&ing| {
            inventory
                .fresh_ingredients
                .iter()
                .any(|range| range.contains(ing))
        })
        .count()
}

fn part02(inventory: &Inventory) -> u64 {
    let mut ranges = inventory.fresh_ingredients.clone();
    ranges.sort();

    let mut merged_ranges: Vec<Range> = Vec::new();

    for &range in &ranges {
        match merged_ranges.last_mut() {
            Some(last) if range.start <= last.end + 1 => {
                last.end = last.end.max(range.end);
            }
            _ => merged_ranges.push(range),
        }
    }

    merged_ranges.iter().map(|range| range.len()).sum()
}

pub fn day05() {
    print_day_header(5);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(5));

    print_section("Part 1 (Example)");
    print_result(
        "This is the number of available fresh ingredients",
        part01(&example_data),
    );

    print_section("Part 1 (Actual)");
    print_result(
        "This is the number of available fresh ingredients",
        part01(&actual_data),
    );

    print_section("Part 2 (Example)");
    print_result(
        "This is the number of possible fresh ingredients",
        part02(&example_data),
    );

    print_section("Part 2 (Actual)");
    print_result(
        "This is the number of possible fresh ingredients",
        part02(&actual_data),
    );
}
