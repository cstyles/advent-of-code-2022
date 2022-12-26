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

    let mut to_trips = vec![];
    let start = Point { y: 1, x: 1 };
    for start_minute in possible_starts(&full_grid, start).into_iter() {
        // let end = Point { y: 4, x: 6 };
        let end = Point { y: 20, x: 150 };
        if let Some(shortest_time) = search(&full_grid, start_minute, start, end) {
            to_trips.push((start_minute, shortest_time - start_minute));
        }
    }

    let mut from_trips = vec![];
    // let start = Point { y: 4, x: 6 };
    let start = Point { y: 20, x: 150 };
    for start_minute in possible_starts(&full_grid, start).into_iter() {
        let end = Point { y: 1, x: 1 };
        if let Some(shortest_time) = search(&full_grid, start_minute, start, end) {
            from_trips.push((start_minute, shortest_time - start_minute));
        }
    }

    for (start_minute, time) in to_trips.into_iter() {
        println!("{start_minute}: {time} (total: {})", start_minute + time);
    }
    println!();
    for (start_minute, time) in from_trips.into_iter() {
        println!("{start_minute}: {time} (total: {})", start_minute + time);
    }

    // Leave on minute 2, takes 330 minutes
    // Arrive at end on minute 332 (%32)
    // Have to wait 1 minute until 333 (%33)
    // Leave on minute 333, takes 297 minutes
    // Arrive at start on 333 + 297 = 630 (%30)
    // Have to wait 1 minute until 631 (%31)
    // Leave on minute 631, takes 311 minutes
    // Arrive at end on 631 + 311 = 942
}

fn search(
    full_grid: &Grid,
    start_minute: usize,
    start_location: Point,
    end_location: Point,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back((start_location, start_minute));

    let mut seen: HashSet<(Point, usize)> = HashSet::default();
    seen.insert((start_location, start_minute));

    while let Some((location, minute)) = queue.pop_front() {
        if location.y == end_location.y && location.x == end_location.x {
            return Some(minute + 1);
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

    None
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

fn possible_starts(full_grid: &Grid, start_location: Point) -> Vec<usize> {
    let (y, x) = (start_location.y, start_location.x);

    full_grid
        .iter()
        .enumerate()
        .filter(|(_minute, snapshot)| !snapshot[y][x])
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
