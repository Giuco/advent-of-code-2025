use crate::utils::{print_day_header, print_result, print_section, read_input};
use std::{collections::HashSet, str::FromStr};

const EXAMPLE: &str = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
/*
 * n: 4
 * 0: 0, 19, 7
 * 1: 2, 13
 * 2: 17, 18
 * 3: 9, 12
 */

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Pos {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns: Vec<i64> = s
            .trim()
            .split(',')
            .map(|n| n.parse().map_err(|e| format!("Parse error: {e}")))
            .collect::<Result<Vec<_>, _>>()?;

        match ns.as_slice() {
            [x, y, z] => Ok(Pos {
                x: *x,
                y: *y,
                z: *z,
            }),
            _ => Err(format!("Invalid Position: {}", s)),
        }
    }
}

impl Pos {
    fn dist_squared(&self, other: &Pos) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

type JunctionBoxes = Vec<Pos>;

fn parse(raw_data: &str) -> JunctionBoxes {
    raw_data
        .trim()
        .lines()
        .map(|line| line.parse().expect("Line not valid"))
        .collect()
}

fn part01(junction_boxes: &JunctionBoxes, n_connections: u16) -> usize {
    let mut distances: Vec<(usize, usize, i64)> = Vec::new();

    for (i, x) in junction_boxes.iter().enumerate() {
        for (j, y) in junction_boxes.iter().enumerate().skip(i + 1) {
            distances.push((i, j, x.dist_squared(y)));
        }
    }

    distances.sort_unstable_by_key(|(_, _, d)| *d);
    let mut connections: Vec<HashSet<usize>> = Vec::new();

    for (i, j, _) in distances.drain(..n_connections as usize) {
        let i_circ = connections.iter().position(|c| c.contains(&i));
        let j_circ = connections.iter().position(|c| c.contains(&j));

        match (i_circ, j_circ) {
            (None, None) => {
                connections.push(HashSet::from([i, j]));
            }
            (None, Some(jc)) => {
                connections[jc].insert(i);
            }
            (Some(ic), None) => {
                connections[ic].insert(j);
            }
            (Some(ic), Some(jc)) if ic == jc => {}
            (Some(ic), Some(jc)) => {
                let j_set = connections.remove(jc);
                let i_idx = if ic > jc { ic - 1 } else { ic };
                connections[i_idx].extend(j_set);
            }
        }
    }

    let mut circuit_sizes: Vec<usize> = connections.iter().map(|c| c.len()).collect();
    circuit_sizes.sort_unstable();
    circuit_sizes.into_iter().rev().take(3).product()
}

fn part02(junction_boxes: &JunctionBoxes) -> Option<i64> {
    let mut distances: Vec<(usize, usize, i64)> = Vec::new();

    for (i, x) in junction_boxes.iter().enumerate() {
        for (j, y) in junction_boxes.iter().enumerate().skip(i + 1) {
            distances.push((i, j, x.dist_squared(y)));
        }
    }

    distances.sort_unstable_by_key(|(_, _, d)| *d);

    let mut connections: Vec<HashSet<usize>> = Vec::new();

    for (i, j, _) in distances.drain(..) {
        let i_circ = connections.iter().position(|c| c.contains(&i));
        let j_circ = connections.iter().position(|c| c.contains(&j));

        match (i_circ, j_circ) {
            (None, None) => {
                connections.push(HashSet::from([i, j]));
            }
            (None, Some(jc)) => {
                connections[jc].insert(i);
            }
            (Some(ic), None) => {
                connections[ic].insert(j);
            }
            (Some(ic), Some(jc)) if ic == jc => {}
            (Some(ic), Some(jc)) => {
                let j_set = connections.remove(jc);
                let i_idx = if ic > jc { ic - 1 } else { ic };
                connections[i_idx].extend(j_set);
            }
        }

        if connections.len() == 1 && connections[0].len() == junction_boxes.len() {
            return Some(junction_boxes[i].x * junction_boxes[j].x);
        }
    }

    None
}

pub fn day08() {
    print_day_header(8);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(8));

    print_section("Part 1 (Example)");
    print_result(
        "The multiplication of the sizes for three largest circuits is",
        part01(&example_data, 10),
    );

    print_section("Part 1 (Actual)");
    print_result(
        "The multiplication of the sizes for three largest circuits is",
        part01(&actual_data, 1000),
    );

    print_section("Part 2 (Example)");
    print_result(
        "If you multiply the x coordinates of the two last junction boxes you need",
        part02(&example_data).expect("Must find solution"),
    );

    print_section("Part 2 (Actual)");
    print_result(
        "If you multiply the x coordinates of the two last junction boxes you need",
        part02(&actual_data).expect("Must find solution"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part01(&matrix, 10), 40)
    }

    #[test]
    fn test_part02_example() {
        let matrix = parse(EXAMPLE);
        assert_eq!(part02(&matrix), Some(25272))
    }
}
