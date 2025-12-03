use crate::utils::{print_day_header, print_result, print_section, read_input};

const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
const PART2_BATTERY_COUNT: usize = 12;

type Bank = Vec<u32>;
type BatteryMatrix = Vec<Bank>;

fn parse(raw_data: &str) -> BatteryMatrix {
    raw_data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Not parseable"))
                .collect()
        })
        .collect()
}

fn highest_jolt(bank: &Bank) -> u32 {
    let (max_idx, &max) = bank
        .iter()
        .enumerate()
        // We can't let it select the last battery otherwise we won't have any options
        .take(bank.len() - 1)
        // Rev is necessary because by default `.max_by_key` will take the last value if there's a tie
        .rev()
        .max_by_key(|(_, value)| *value)
        .expect("Bank must have at least one element");

    let &second_max = bank
        .iter()
        // We need to get a battery positioned right to the one we already selected
        .skip(max_idx + 1)
        .max()
        .expect("Bank must have at least two elements");

    max * 10 + second_max
}

fn highest_jolt_n(bank: &Bank, count: usize) -> u64 {
    let mut max_battery_idx = 0;
    let mut selected_batteries: Vec<u32> = Vec::with_capacity(count);

    for n in 1..=count {
        let space_buffer = count - n;

        let (idx, &max) = bank
            .iter()
            .enumerate()
            // We can't select a battery positioned to left of the last selected battery.
            .skip(max_battery_idx)
            .rev()
            // This is necessary so we have space to select all the `n` batteries, otherwise we could eagerly
            // select a big one near the end and then we wouldn't have enough batteries anymore
            .skip(space_buffer)
            .max_by_key(|(_, value)| *value)
            .expect("Not enough batteries in bank");

        max_battery_idx = idx + 1;
        selected_batteries.push(max);
    }

    selected_batteries
        .iter()
        .fold(0, |acc, &digit| (acc * 10) + (digit as u64))
}

fn part01(data: &BatteryMatrix) -> u32 {
    data.iter().map(highest_jolt).sum()
}

fn part02(data: &BatteryMatrix) -> u64 {
    data.iter()
        .map(|bank| highest_jolt_n(bank, PART2_BATTERY_COUNT))
        .sum()
}

pub fn day03() {
    print_day_header(3);

    let example_data = parse(EXAMPLE);
    let actual_data = parse(&read_input(3));

    print_section("Part 1 (Example)");
    print_result("The total output joltage is: ", part01(&example_data));

    print_section("Part 1 (Actual)");
    print_result("The total output joltage is: ", part01(&actual_data));

    print_section("Part 2 (Example)");
    print_result("The total output joltage is: ", part02(&example_data));

    print_section("Part 2 (Actual)");
    print_result("The total output joltage is: ", part02(&actual_data));
}
