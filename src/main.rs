mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod utils;

use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use std::env;

const DEFAULT_DAY: usize = 5;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT_DAY)
    } else {
        DEFAULT_DAY
    };

    match day {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        _ => println!("Day {} not implemented yet!", day),
    }
}
