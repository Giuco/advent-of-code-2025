const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

const DIAL_SIZE: i32 = 100;
const STARTING_POSITION: i32 = 50;
const TARGET_POSITION: i32 = 0;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Rotation {
    direction: Direction,
    times: i32,
}

fn parse(raw_data: &str) -> Vec<Rotation> {
    raw_data
        .lines()
        .map(|r| {
            let (raw_direction, raw_times) = r.split_at(1);
            let times = raw_times.parse().expect("Failed to parse number");
            let direction = match raw_direction {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction: {}", raw_direction),
            };

            Rotation { direction, times }
        })
        .collect()
}

fn apply_rotation(position: i32, rotation: &Rotation) -> i32 {
    let delta = match rotation.direction {
        Direction::Left => -rotation.times,
        Direction::Right => rotation.times,
    };

    (position + delta).rem_euclid(DIAL_SIZE)
}

fn count_zero_crossings(start_position: i32, rotation: &Rotation) -> i32 {
    let steps_to_first_zero = match (start_position, rotation.direction) {
        (0, _) => DIAL_SIZE,
        (pos, Direction::Right) => DIAL_SIZE - pos,
        (pos, Direction::Left) => pos,
    };

    if rotation.times < steps_to_first_zero {
        0
    } else {
        1 + (rotation.times - steps_to_first_zero) / DIAL_SIZE
    }
}

fn part01(data: &[Rotation]) {
    let mut position = STARTING_POSITION;
    let mut zero_count = 0;

    for rotation in data {
        position = apply_rotation(position, rotation);

        if position == TARGET_POSITION {
            zero_count += 1;
        }
    }

    println!("The dial points to zero: {} times", zero_count);
}

fn part02(data: &[Rotation]) {
    let mut position = STARTING_POSITION;
    let mut zero_count = 0;

    for rotation in data {
        zero_count += count_zero_crossings(position, &rotation);
        position = apply_rotation(position, rotation);
    }

    println!("The dial passed through zero: {} times", zero_count);
}

pub fn day01() {
    let example_data = parse(EXAMPLE);
    let actual_data = parse(include_str!("../data/day01.txt"));

    println!("-----------------");
    println!("Example 01");
    println!("-----------------");
    part01(&example_data);
    println!("-----------------");
    part01(&actual_data);

    println!("=================");
    println!("=================");

    println!("Example 02");
    println!("-----------------");
    part02(&example_data);
    println!("-----------------");
    part02(&actual_data);
    println!("-----------------");
}
