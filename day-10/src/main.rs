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

#[derive(Default, Debug, Clone, Copy)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn advance(&mut self) {
        self.column += 1;

        // TODO: ugh
        if self.column >= 40 {
            self.column = 0;
            self.row += 1;

            if self.row >= 6 {
                self.row = 0;
            }
        }
    }

    fn in_range(&self, x: i32) -> bool {
        (x - 1..=x + 1).contains(&(self.column as i32))
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

    let mut screen = [[false; 40]; 6];
    let mut pixel_being_drawn = Point::default();

    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                signal_strengths.push(x * clock);
            }

            screen[pixel_being_drawn.row][pixel_being_drawn.column] = pixel_being_drawn.in_range(x);
            pixel_being_drawn.advance();
        }

        if let Instruction::Add(v) = instruction {
            x += v;
        }
    }

    let part1: i32 = signal_strengths.into_iter().sum();
    println!("part1 = {part1}");

    draw(screen);
}

fn draw(screen: [[bool; 40]; 6]) {
    for row in screen {
        for pixel in row {
            match pixel {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!();
    }
}
