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

#[derive(Debug, Clone, Copy)]
struct Screen {
    grid: [[bool; Self::WIDTH]; Self::HEIGHT],
    pixel_being_drawn: Point,
}

impl Screen {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    fn advance(&mut self) {
        self.pixel_being_drawn.advance();
    }

    fn draw(&mut self, sprite_position: i32) {
        self.grid[self.pixel_being_drawn.row][self.pixel_being_drawn.column] =
            self.pixel_being_drawn.in_range(sprite_position);
        self.advance();
    }

    fn print(&self) {
        for row in self.grid {
            for pixel in row {
                match pixel {
                    true => print!("#"),
                    false => print!("."),
                };
            }
            println!();
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            grid: [[false; Self::WIDTH]; Self::HEIGHT],
            pixel_being_drawn: Point::default(),
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
    let mut screen = Screen::default();

    for instruction in instructions {
        for _ in 0..instruction.cycles() {
            clock += 1;
            if (clock - 20) % 40 == 0 {
                signal_strengths.push(x * clock);
            }

            screen.draw(x);
        }

        if let Instruction::Add(v) = instruction {
            x += v;
        }
    }

    let part1: i32 = signal_strengths.into_iter().sum();
    println!("part1 = {part1}");

    screen.print();
}
