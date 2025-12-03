mod day01;
mod day02;
mod day03;
mod utils;

use day01::day01;
use day02::day02;
use day03::day03;
use std::env;

const DEFAULT_DAY: usize = 3;

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
        _ => println!("Day {} not implemented yet!", day),
    }
}
