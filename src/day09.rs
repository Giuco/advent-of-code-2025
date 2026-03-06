use crate::utils::{print_day_header, print_result, print_section, read_input};
use std::{collections::HashSet, str::FromStr};

const EXAMPLE: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns: Vec<usize> = s
            .trim()
            .split(',')
            .map(|n| n.parse().map_err(|e| format!("Parse error: {e}")))
            .collect::<Result<Vec<_>, _>>()?;

        match ns.as_slice() {
            [x, y] => Ok(Pos { x: *x, y: *y }),
            _ => Err(format!("Invalid Position: {}", s)),
        }
    }
}

impl Pos {
    fn area(&self, other: &Pos) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

type RedTiles = Vec<Pos>;

fn parse(raw_data: &str) -> RedTiles {
    raw_data
        .trim()
        .lines()
        .map(|line| line.parse().expect("Line not valid"))
        .collect()
}

fn part01(red_tiles: &RedTiles) -> usize {
    let mut max = 0;

    for (i, a) in red_tiles.iter().enumerate() {
        for b in red_tiles.iter().skip(i + 1) {
            let area = a.area(b);
            max = max.max(area)
        }
    }

    max
}

fn part02(red_tiles: &RedTiles) -> usize {
    todo!()
}

pub fn day09() {
    print_day_header(9);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(9));

    print_section("Part 1 (Example)");
    print_result(
        "The maximum are for the red tiles is",
        part01(&example_data),
    );

    print_section("Part 1 (Actual)");
    print_result("The maximum are for the red tiles is", part01(&actual_data));

    // print_section("Part 2 (Example)");
    // print_result(
    //     "If you multiply the x coordinates of the two last junction boxes you need",
    //     part02(&example_data).expect("Must find solution"),
    // );

    // print_section("Part 2 (Actual)");
    // print_result(
    //     "If you multiply the x coordinates of the two last junction boxes you need",
    //     part02(&actual_data).expect("Must find solution"),
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part01(&matrix), 50)
    }

    #[test]
    fn test_part02_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part02(&matrix), 50)
    }
}
