use crate::utils::{print_day_header, print_result, print_section, read_input};

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

#[derive(Debug, Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn parse_range(s: &str) -> Result<Range, String> {
    let (raw_start, raw_end) = s
        .split_once('-')
        .ok_or_else(|| format!("Not a valid range: {}", s))?;

    Ok(Range {
        start: raw_start
            .trim()
            .parse()
            .map_err(|_| format!("Not a valid number: {}", raw_start))?,
        end: raw_end
            .trim()
            .parse()
            .map_err(|_| format!("Not a valid number: {}", raw_end))?,
    })
}

fn parse(raw_data: &str) -> Vec<Range> {
    raw_data
        .split(',')
        .map(|s| parse_range(s).expect("Failed to parse range"))
        .collect()
}

fn is_mirror_password(password: u64) -> bool {
    let password_string = password.to_string();
    let len = password_string.len();

    if len % 2 != 0 {
        return false;
    }

    let (left, right) = password_string.split_at(len / 2);
    left == right
}

fn part01(data: &[Range]) -> u64 {
    data.iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&password| is_mirror_password(password))
        .sum()
}

fn has_repeating_pattern(password: u64) -> bool {
    let password_string = password.to_string();
    let len = password_string.len();

    (1..len).any(|split_size| {
        if len % split_size != 0 {
            return false;
        }

        let first_chunk = &password_string[..split_size];

        password_string
            .as_bytes()
            .chunks(split_size)
            .all(|chunk| chunk == first_chunk.as_bytes())
    })
}

fn part02(data: &[Range]) -> u64 {
    data.iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&password| has_repeating_pattern(password))
        .sum()
}

pub fn day02() {
    print_day_header(2);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(2));

    print_section("Part 1 (Example)");
    print_result("Sum of mirror passwords", part01(&example_data));

    print_section("Part 1 (Actual)");
    print_result("Sum of mirror passwords", part01(&actual_data));

    print_section("Part 2 (Example)");
    print_result("Sum of repeating pattern passwords", part02(&example_data));

    print_section("Part 2 (Actual)");
    print_result("Sum of repeating pattern passwords", part02(&actual_data));
}
