fn main() {
    let input = include_str!("../input.txt");

    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();

    elves.sort();

    let part1 = elves.last().unwrap();
    let part2: u32 = elves.iter().rev().take(3).sum();

    println!("part1 = {part1}");
    println!("part2 = {part2}");
}
