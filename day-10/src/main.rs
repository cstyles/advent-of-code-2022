use std::fs::read_to_string;

enum Instruction {
    Noop,
    Add(i32),
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match *self {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        match input.split_once(' ') {
            None => Self::Noop,
            Some((_addx, value)) => Self::Add(value.parse().unwrap()),
        }
    }
}

fn main() {
    let input = match std::env::var("TEST") {
        Ok(number) => read_to_string(format!("test_input{number}.txt")),
        Err(_) => read_to_string("input.txt"),
    }
    .unwrap();

    let instructions = input.lines().map(Instruction::from);
    let mut clock = 0;
    let mut x = 1;
    let mut signal_strengths = vec![];

    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                signal_strengths.push(x * clock);
            }
        }

        if let Instruction::Add(v) = instruction {
            x += v;
        }
    }

    let part1: i32 = signal_strengths.into_iter().sum();
    println!("part1 = {part1}");
}
