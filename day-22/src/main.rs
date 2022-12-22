use Action::*;
use Direction::*;
use Tile::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Border,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Open,
            '#' => Wall,
            ' ' => Border,
            _ => panic!("bad tile: {c}"),
        }
    }
}

#[derive(Debug)]
enum Action {
    Steps(u8),
    TurnClockwise,
    TurnCounterClockwise,
}

impl Action {
    fn parse_steps(string: &str) -> Self {
        Self::Steps(string.parse().unwrap())
    }

    fn parse_turn(string: &str) -> Self {
        match string {
            "R" => TurnClockwise,
            "L" => TurnCounterClockwise,
            _ => panic!("not a turn: {string}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn turn(&mut self, turn: Action) {
        *self = match (turn, &self) {
            (TurnClockwise, Right) => Down,
            (TurnClockwise, Left) => Up,
            (TurnClockwise, Up) => Right,
            (TurnClockwise, Down) => Left,
            (TurnCounterClockwise, Right) => Up,
            (TurnCounterClockwise, Left) => Down,
            (TurnCounterClockwise, Up) => Left,
            (TurnCounterClockwise, Down) => Right,
            (Steps(_), _) => panic!("AAAAA"),
        };
    }

    fn to_number(self) -> isize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

fn parse_actions(string: &str) -> Vec<Action> {
    let mut wow = vec![];

    // The input always ends with some steps
    let (string, last_steps) = string.split_at(string.len() - 1);
    let last_steps = Action::parse_steps(last_steps);

    for steps_turn_pair in string.split_inclusive(['R', 'L']) {
        let (steps, turn) = steps_turn_pair.split_at(steps_turn_pair.len() - 1);
        wow.push(Action::parse_steps(steps));
        wow.push(Action::parse_turn(turn));
    }

    wow.push(last_steps);
    wow
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Open => '.',
            Wall => '#',
            Border => ' ',
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point {
    y: isize,
    x: isize,
}

impl Point {
    fn move_(&mut self, mut steps: u8, facing: Direction, grid: &[Vec<Tile>]) {
        while steps > 0 {
            let next_point = self.next_point(facing, grid);
            match lookup_unchecked(next_point, grid) {
                Open => *self = next_point,
                Wall => break,
                Border => unreachable!(),
            }

            steps -= 1;
        }
    }

    fn next_point(self, facing: Direction, grid: &[Vec<Tile>]) -> Self {
        let next_point = self.next_point_naive(facing);

        match lookup(next_point, grid) {
            Some(Open) | Some(Wall) => next_point,
            Some(Border) | None => self.wrap(facing, grid),
        }
    }

    fn next_point_naive(self, facing: Direction) -> Self {
        match facing {
            Right => self.right(),
            Left => self.left(),
            Up => self.up(),
            Down => self.down(),
        }
    }

    fn wrap(self, facing: Direction, grid: &[Vec<Tile>]) -> Self {
        match facing {
            Right => {
                let x = grid[self.y as usize]
                    .iter()
                    .position(|tile| *tile != Border)
                    .unwrap() as isize;
                Point { x, ..self }
            }
            Left => {
                let x = grid[self.y as usize]
                    .iter()
                    .rposition(|tile| *tile != Border)
                    .unwrap() as isize;
                Point { x, ..self }
            }
            Up => {
                let y = grid
                    .iter()
                    .rposition(|row| {
                        row.get(self.x as usize)
                            .map(|tile| *tile != Border)
                            .unwrap_or(false)
                    })
                    .unwrap() as isize;

                Point { y, ..self }
            }
            Down => {
                let y = grid
                    .iter()
                    .position(|row| {
                        row.get(self.x as usize)
                            .map(|tile| *tile != Border)
                            .unwrap_or(false)
                    })
                    .unwrap() as isize;

                Point { y, ..self }
            }
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
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let (grid, actions) = input.split_once("\n\n").unwrap();
    let grid: Vec<Vec<Tile>> = grid
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();
    let actions = parse_actions(actions.trim_end());

    let x = grid[0].iter().position(|tile| *tile == Open).unwrap() as isize;
    let mut location = Point { y: 0, x };
    let mut facing = Direction::Right;

    for action in actions.into_iter() {
        match action {
            Steps(steps) => location.move_(steps, facing, &grid),
            TurnClockwise | TurnCounterClockwise => facing.turn(action),
        }
    }

    println!("part1 = {}", answer(location, facing));
}

#[allow(unused)]
fn print_grid(grid: &[Vec<Tile>]) {
    for row in grid {
        let row: String = row.iter().map(Tile::as_char).collect();
        println!("{row}");
    }
}

fn answer(point: Point, facing: Direction) -> isize {
    let row = point.y + 1;
    let column = point.x + 1;
    let facing = facing.to_number();

    1000 * row + 4 * column + facing
}

fn lookup(point: Point, grid: &[Vec<Tile>]) -> Option<Tile> {
    grid.get(point.y as usize)
        .and_then(|row| row.get(point.x as usize))
        .copied()
}

fn lookup_unchecked(point: Point, grid: &[Vec<Tile>]) -> Tile {
    grid[point.y as usize][point.x as usize]
}
