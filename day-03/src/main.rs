fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

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

fn score(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 64 + 26
    } else {
        c as u32 - 96
    }
}
