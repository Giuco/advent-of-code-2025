mod day01;
mod day02;
mod utils;

use day01::day01;
use day02::day02;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };

    match day {
        1 => day01(),
        2 => day02(),
        _ => println!("Day {} not implemented yet!", day),
    }
}
