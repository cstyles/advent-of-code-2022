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
    fn turn(&mut self, turn: &Action) {
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
    fn move_part1(&mut self, mut steps: u8, facing: Direction, grid: &[Vec<Tile>]) {
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

    fn move_part2(
        mut self,
        mut steps: u8,
        mut facing: Direction,
        grid: &[Vec<Tile>],
    ) -> (Self, Direction) {
        while steps > 0 {
            let (new_location, new_facing) = self.next_state(facing, grid);
            match lookup_unchecked(new_location, grid) {
                Open => {
                    self = new_location;
                    facing = new_facing;
                }
                Wall => break,
                Border => unreachable!(),
            }

            steps -= 1;
        }

        (self, facing)
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

    fn wrap2(self, facing: Direction, grid: &[Vec<Tile>]) -> (Self, Direction) {
        // let face_size = (grid.len() / 3) as isize;
        let face_size = (grid[0].len() / 3) as isize;

        match facing {
            Right => {
                if self.y < face_size {
                    // on 6, go to 5, facing left
                    let x = face_size * 2 - 1;
                    let y = face_size - self.y - 1; // mirror
                    let y = y + face_size * 2;
                    (Self { y, x }, Left)
                } else if self.y < face_size * 2 {
                    // on 4, go to 6, facing up
                    let y = face_size - 1;
                    let x = self.y - face_size;
                    let x = x + face_size * 2;
                    (Self { y, x }, Up)
                } else if self.y < face_size * 3 {
                    // on 5, go to 6, facing left
                    let x = face_size * 3 - 1;
                    let y = self.y - face_size * 2;
                    let y = face_size - y - 1; // mirror
                    (Self { y, x }, Left)
                } else {
                    // on 2, go to 5, facing up
                    let y = face_size * 3 - 1;
                    let x = self.y - face_size * 3;
                    let x = x + face_size;
                    (Self { y, x }, Up)
                }
            }
            Left => {
                if self.y < face_size {
                    // on 1, go to 3, facing right
                    let x = 0;
                    let y = face_size - self.y - 1; // mirror
                    let y = y + face_size * 2;
                    (Self { y, x }, Right)
                } else if self.y < face_size * 2 {
                    // on 4, go to 3, facing down
                    let y = face_size * 2;
                    let x = self.y - face_size;
                    (Self { y, x }, Down)
                } else if self.y < face_size * 3 {
                    // on 3, go to 1, facing right
                    let x = face_size;
                    let y = self.y - face_size * 2;
                    let y = face_size - y - 1; // mirror
                    (Self { y, x }, Right)
                } else {
                    // on 2, go to 1, facing down
                    let y = 0;
                    let x = self.y - face_size * 3;
                    let x = x + face_size;
                    (Self { y, x }, Down)
                }
            }
            Up => {
                if self.x < face_size {
                    // on 3, go to 4, facing right
                    let x = face_size;
                    let y = self.x + face_size;
                    (Self { y, x }, Right)
                } else if self.x < face_size * 2 {
                    // on 1, go to 2, facing right
                    let x = 0;
                    let y = self.x - face_size;
                    let y = y + face_size * 3;
                    (Self { y, x }, Right)
                } else {
                    // on 6, go to 2, facing up
                    let y = face_size * 4 - 1;
                    let x = self.x - face_size * 2;
                    (Self { y, x }, Up)
                }
            }
            Down => {
                if self.x < face_size {
                    // on 2, go to 6, facing down
                    let y = 0;
                    let x = self.x + face_size * 2;
                    (Self { y, x }, Down)
                } else if self.x < face_size * 2 {
                    // on 5, go to 2, facing left
                    let x = face_size - 1;
                    let y = self.x - face_size;
                    let y = y + face_size * 3;
                    (Self { y, x }, Left)
                } else {
                    // on 6, go to 4, facing left
                    let x = face_size * 2 - 1;
                    let y = self.x - face_size * 2;
                    let y = y + face_size;
                    (Self { y, x }, Left)
                }
            }
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

    fn next_state(self, facing: Direction, grid: &[Vec<Tile>]) -> (Self, Direction) {
        let next_point = self.next_point_naive(facing);

        match lookup(next_point, grid) {
            Some(Open) | Some(Wall) => (next_point, facing),
            Some(Border) | None => self.wrap2(facing, grid),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let (grid, actions) = input.split_once("\n\n").unwrap();
    let grid: Vec<Vec<Tile>> = grid
        .lines()
        .map(|line| line.chars().map(Tile::from).collect())
        .collect();
    let actions = parse_actions(actions.trim_end());

    let x = grid[0].iter().position(|tile| *tile == Open).unwrap() as isize;
    let location = Point { y: 0, x };
    let facing = Direction::Right;

    part1(location, facing, &actions, &grid);
    part2(location, facing, &actions, &grid);
}

fn part1(mut location: Point, mut facing: Direction, actions: &[Action], grid: &[Vec<Tile>]) {
    for action in actions {
        match action {
            Steps(steps) => location.move_part1(*steps, facing, grid),
            TurnClockwise | TurnCounterClockwise => facing.turn(action),
        }
    }

    println!("part1 = {}", answer(location, facing));
}

fn part2(mut location: Point, mut facing: Direction, actions: &[Action], grid: &[Vec<Tile>]) {
    for action in actions {
        match action {
            Steps(steps) => {
                let (new_location, new_facing) = location.move_part2(*steps, facing, grid);
                location = new_location;
                facing = new_facing;
            }
            TurnClockwise | TurnCounterClockwise => facing.turn(action),
        }
    }

    println!("part2 = {}", answer(location, facing));
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

mod tests {
    use super::*;

    fn init() -> Vec<Vec<Tile>> {
        let input = include_str!("../crafted2.txt");

        let (grid, _actions) = input.split_once("\n\n").unwrap();
        grid.lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect()
    }

    #[test]
    fn up_from_3() {
        let grid = init();

        let location = Point { y: 8, x: 0 };
        let facing = Up;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 4, x: 4 });
        assert_eq!(facing, Right);

        let location = Point { y: 8, x: 3 };
        let facing = Up;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 7, x: 5 });
        assert_eq!(facing, Right);
    }

    #[test]
    fn up_from_1() {
        let grid = init();

        let location = Point { y: 0, x: 4 };
        let facing = Up;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 12, x: 0 });
        assert_eq!(facing, Right);

        let location = Point { y: 0, x: 7 };
        let facing = Up;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 15, x: 1 });
        assert_eq!(facing, Right);
    }

    #[test]
    fn up_from_6() {
        let grid = init();

        let location = Point { y: 0, x: 8 };
        let facing = Up;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 15, x: 0 });
        assert_eq!(facing, Up);

        let location = Point { y: 0, x: 11 };
        let facing = Up;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 14, x: 3 });
        assert_eq!(facing, Up);
    }

    #[test]
    fn down_from_2() {
        let grid = init();

        let location = Point { y: 15, x: 0 };
        let facing = Down;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 0, x: 8 });
        assert_eq!(facing, Down);

        let location = Point { y: 15, x: 3 };
        let facing = Down;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 1, x: 11 });
        assert_eq!(facing, Down);
    }

    #[test]
    fn down_from_5() {
        let grid = init();

        let location = Point { y: 11, x: 4 };
        let facing = Down;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 12, x: 3 });
        assert_eq!(facing, Left);

        let location = Point { y: 11, x: 7 };
        let facing = Down;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 15, x: 2 });
        assert_eq!(facing, Left);
    }

    #[test]
    fn down_from_6() {
        let grid = init();

        let location = Point { y: 3, x: 8 };
        let facing = Down;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 4, x: 7 });
        assert_eq!(facing, Left);

        let location = Point { y: 3, x: 11 };
        let facing = Down;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 7, x: 6 });
        assert_eq!(facing, Left);
    }

    #[test]
    fn left_from_1() {
        let grid = init();

        let location = Point { y: 0, x: 4 };
        let facing = Left;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 11, x: 0 });
        assert_eq!(facing, Right);

        let location = Point { y: 3, x: 4 };
        let facing = Left;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 8, x: 1 });
        assert_eq!(facing, Right);
    }

    #[test]
    fn left_from_4() {
        let grid = init();

        let location = Point { y: 4, x: 4 };
        let facing = Left;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 8, x: 0 });
        assert_eq!(facing, Down);

        let location = Point { y: 7, x: 4 };
        let facing = Left;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 9, x: 3 });
        assert_eq!(facing, Down);
    }

    #[test]
    fn left_from_2() {
        let grid = init();

        let location = Point { y: 12, x: 0 };
        let facing = Left;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 0, x: 4 });
        assert_eq!(facing, Down);

        let location = Point { y: 15, x: 0 };
        let facing = Left;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 1, x: 7 });
        assert_eq!(facing, Down);
    }

    #[test]
    fn left_from_3() {
        let grid = init();

        let location = Point { y: 8, x: 0 };
        let facing = Left;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 3, x: 4 });
        assert_eq!(facing, Right);

        let location = Point { y: 11, x: 0 };
        let facing = Left;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 0, x: 5 });
        assert_eq!(facing, Right);
    }

    #[test]
    fn right_from_6() {
        let grid = init();

        let location = Point { y: 0, x: 11 };
        let facing = Right;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 11, x: 7 });
        assert_eq!(facing, Left);

        let location = Point { y: 3, x: 11 };
        let facing = Right;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 8, x: 6 });
        assert_eq!(facing, Left);
    }

    #[test]
    fn right_from_4() {
        let grid = init();

        let location = Point { y: 4, x: 7 };
        let facing = Right;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 3, x: 8 });
        assert_eq!(facing, Up);

        let location = Point { y: 7, x: 7 };
        let facing = Right;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 2, x: 11 });
        assert_eq!(facing, Up);
    }

    #[test]
    fn right_from_5() {
        let grid = init();

        let location = Point { y: 8, x: 7 };
        let facing = Right;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 3, x: 11 });
        assert_eq!(facing, Left);

        let location = Point { y: 11, x: 7 };
        let facing = Right;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 0, x: 10 });
        assert_eq!(facing, Left);
    }

    #[test]
    fn right_from_2() {
        let grid = init();

        let location = Point { y: 12, x: 3 };
        let facing = Right;
        let actions = vec![Steps(1)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 11, x: 4 });
        assert_eq!(facing, Up);

        let location = Point { y: 15, x: 4 };
        let facing = Right;
        let actions = vec![Steps(2)];
        let (location, facing) = part2(location, facing, actions, &grid);
        assert_eq!(location, Point { y: 10, x: 7 });
        assert_eq!(facing, Up);
    }
}
