fn main() {
    let input = include_str!("../input.txt");

    let part1: u32 = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap();

    println!("part1 = {part1}");
}
