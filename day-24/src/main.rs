use std::collections::{HashSet, VecDeque};

use Direction::*;

type Grid = Vec<Snapshot>;
type Snapshot = Vec<Vec<bool>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn shift(self, direction: Direction) -> Self {
        match direction {
            Up => self.up(),
            Down => self.down(),
            Left => self.left(),
            Right => self.right(),
        }
        .wrap()
    }

    fn wrap(mut self) -> Self {
        if self.y == 0 {
            self.y = 20;
            // self.y = 4;
        } else if self.y > 20 {
            // } else if self.y > 4 {
            self.y = 1;
        }

        if self.x == 0 {
            self.x = 150;
            // self.x = 6;
        } else if self.x > 150 {
            // } else if self.x > 6 {
            self.x = 1;
        }

        self
    }

    fn neighbors(&self) -> Vec<Self> {
        [self.up(), self.down(), self.left(), self.right(), *self]
            .into_iter()
            .filter(Self::valid)
            .collect()
    }

    fn valid(&self) -> bool {
        !(self.x == 0 || self.x > 150 || self.y == 0 || self.y > 20)
        // !(self.x == 0 || self.x > 6 || self.y == 0 || self.y > 4)
    }

    fn left(self) -> Self {
        Self {
            x: self.x - 1,
            ..self
        }
    }

    fn right(self) -> Self {
        Self {
            x: self.x + 1,
            ..self
        }
    }

    fn up(self) -> Self {
        Self {
            y: self.y - 1,
            ..self
        }
    }

    fn down(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(character: char) -> Self {
        match character {
            '>' => Right,
            '<' => Left,
            '^' => Up,
            'v' => Down,
            _ => panic!("bad char: {character}"),
        }
    }
}

impl Direction {
    #[allow(unused)]
    fn as_char(self) -> char {
        match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    point: Point,
    direction: Direction,
}

impl Blizzard {
    fn advance(&mut self) {
        self.point = self.point.shift(self.direction);
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut blizzards = vec![];
    let number_of_rows = input.lines().count() - 2;
    let number_of_columns = input.lines().next().unwrap().len() - 2;

    for (y, row) in input.lines().enumerate().skip(1).take(number_of_rows) {
        for (x, c) in row
            .chars()
            .enumerate()
            .skip(1)
            .take(number_of_columns)
            .filter(|(_, c)| *c != '.')
        {
            let direction = Direction::from(c);
            let point = Point { y, x };
            let blizzard = Blizzard { direction, point };

            blizzards.push(blizzard);
        }
    }

    let full_grid: Grid = populate_grid(&mut blizzards, number_of_rows, number_of_columns);

    let location = Point { y: 1, x: 1 };
    // let minute: usize = 2;
    let mut queue = VecDeque::new();
    let mut seen: HashSet<(Point, usize)> = HashSet::default();

    // print_snapshot(&full_grid[0]);
    // println!();
    // print_snapshot(&full_grid[1]);
    // println!();
    // print_snapshot(&full_grid[2]);
    // println!();

    for minute in possible_starts(&full_grid).into_iter().take(1) {
        queue.push_back((location, minute));
        seen.insert((location, minute));
    }

    while let Some((location, minute)) = queue.pop_front() {
        if location.y == number_of_rows && location.x == number_of_columns {
            println!("part1 = {}", minute + 1);
            break;
        }

        let grid = &full_grid[(minute + 1) % full_grid.len()];
        for neighbor in location.neighbors() {
            if grid[neighbor.y][neighbor.x] {
                continue;
            }

            if !seen.contains(&(neighbor, (minute + 1) % full_grid.len())) {
                queue.push_back((neighbor, minute + 1));
                seen.insert((neighbor, (minute + 1) % full_grid.len()));
            }
        }

        seen.insert((location, (minute + 1) % full_grid.len()));
    }
}

fn populate_grid(
    blizzards: &mut [Blizzard],
    number_of_rows: usize,
    number_of_columns: usize,
) -> Grid {
    let mut final_grid = vec![];

    for _minute in 0..300 {
    // for _minute in 0..12 {
        let mut grid = vec![vec![false; number_of_columns + 1]; number_of_rows + 1];

        for blizzard in blizzards.iter_mut() {
            grid[blizzard.point.y][blizzard.point.x] = true;
            blizzard.advance();
        }

        final_grid.push(grid);
    }

    final_grid
}

fn possible_starts(full_grid: &Grid) -> Vec<usize> {
    full_grid
        .iter()
        .enumerate()
        .filter(|(_minute, snapshot)| !snapshot[1][1])
        .map(|(minute, _snapshot)| minute)
        .collect()
}

#[allow(unused)]
fn print_snapshot(snapshot: &Snapshot) {
    for row in snapshot.iter().skip(1) {
        let row: String = row
            .iter()
            .skip(1)
            .map(|b| match b {
                true => '#',
                false => '.',
            })
            .collect();

        println!("{row}");
    }
}

#[allow(unused)]
fn print_blizzards(blizzards: &[Blizzard]) {
    let mut grid = vec![vec!['.'; 7]; 5];

    for blizzard in blizzards {
        grid[blizzard.point.y][blizzard.point.x] = blizzard.direction.as_char();
    }

    for row in grid.into_iter().skip(1) {
        let row: String = row.iter().skip(1).collect();
        println!("{row}");
    }
}
