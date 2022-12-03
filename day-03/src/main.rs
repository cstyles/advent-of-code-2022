use std::collections::HashSet;

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let part1: u32 = input
        .lines()
        .map(|line| {
            let midpoint = line.len() / 2;
            let first = &line[0..midpoint];
            let second = &line[midpoint..];

            let mut seen = std::collections::HashSet::with_capacity(midpoint);
            for c in first.chars() {
                seen.insert(c);
            }

            second.chars().find(|c| seen.contains(c)).unwrap()
        })
        .map(score)
        .sum();

    println!("part1 = {part1}");
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    let part2: u32 = lines
        .chunks(3)
        .map(|chunk| {
            let seen_first: HashSet<char> = chunk[0].chars().collect();
            let seen_second: HashSet<char> = chunk[1].chars().collect();
            let seen_third: HashSet<char> = chunk[2].chars().collect();

            *seen_first
                .intersection(&seen_second)
                .find(|c| seen_third.contains(c))
                .unwrap()
        })
        .map(score)
        .sum();

    println!("part2 = {part2}");
}

fn score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}
