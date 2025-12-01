use std::fs;

pub fn read_input(day: u32) -> String {
    let filename = format!("data/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

pub fn print_day_header(day: u32) {
    println!("\n╔════════════════════════════════╗");
    println!("║        Day \x1b[1m{:02}\x1b[0m Solutions        ║", day);
    println!("╚════════════════════════════════╝\n");
}

pub fn print_section(title: &str) {
    println!("\n┌─ {} ─┐", title);
}

pub fn print_result(label: &str, value: impl std::fmt::Display) {
    println!("  {}: \x1b[1m{}\x1b[0m", label, value);
}
