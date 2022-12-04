use std::ops::RangeInclusive;

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let part1 = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first, second)| (parse_range(first), parse_range(second)))
        .filter(|(first, second)| full_overlap(first, second))
        .count();

    println!("part1 = {part1}");
}

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let (start, end) = input.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
    RangeInclusive::new(start, end)
}

fn full_overlap(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    let r1_contains_r2 = r1.contains(r2.start()) && r1.contains(r2.end());
    let r2_contains_r1 = r2.contains(r1.start()) && r2.contains(r1.end());

    r1_contains_r2 || r2_contains_r1
}
