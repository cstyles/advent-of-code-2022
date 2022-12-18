use Direction::*;
use PieceKind::*;
use Tile::*;

const WIDTH: usize = 7;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Left,
            '>' => Right,
            _ => panic!("invalid direction: {c}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PieceKind {
    Line,
    Plus,
    Ell,
    Column,
    Square,
}
impl PieceKind {
    fn height(&self) -> usize {
        match self {
            Line => 1,
            Plus => 3,
            Ell => 3,
            Column => 4,
            Square => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Air
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Air => '.',
            Rock => '#',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn down(self) -> Option<Self> {
        let y = self.y.checked_sub(1)?;

        Some(Self { y, ..self })
    }

    fn left(self) -> Option<Self> {
        let x = self.x.checked_sub(1)?;

        Some(Self { x, ..self })
    }

    fn right(self) -> Option<Self> {
        let x = self.x + 1;
        if x >= WIDTH {
            None
        } else {
            Some(Self {
                x: self.x + 1,
                ..self
            })
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Piece {
    kind: PieceKind,
    position: Point,
}

impl Piece {
    fn new(kind: PieceKind, top: usize) -> Piece {
        let y = top + 2 + kind.height();

        Piece {
            kind,
            position: Point { x: 2, y },
        }
    }

    fn descend(&mut self, tower: &Tower) -> Option<()> {
        let would_be = Self {
            position: self.position.down()?,
            ..*self
        };

        if !would_be.legal() {
            return None;
        }

        if tower.collides(would_be) {
            None
        } else {
            *self = would_be;
            Some(())
        }
    }

    fn shift(&mut self, direction: Direction, tower: &Tower) -> Option<()> {
        let new_position = match direction {
            Left => self.position.left()?,
            Right => self.position.right()?,
        };

        let would_be = Self {
            position: new_position,
            ..*self
        };

        if !would_be.legal() {
            return None;
        }

        if tower.collides(would_be) {
            None
        } else {
            *self = would_be;
            Some(())
        }
    }

    fn points(&self) -> Option<Vec<Point>> {
        match self.kind {
            Line => self.line_points(),
            Plus => self.plus_points(),
            Ell => self.ell_points(),
            Column => self.column_points(),
            Square => self.square_points(),
        }
    }

    fn line_points(&self) -> Option<Vec<Point>> {
        let p = Some(self.position);

        [
            p,
            p.and_then(Point::right),
            p.and_then(Point::right).and_then(Point::right),
            p.and_then(Point::right)
                .and_then(Point::right)
                .and_then(Point::right),
        ]
        .into_iter()
        .collect()
    }

    fn plus_points(&self) -> Option<Vec<Point>> {
        let p = Some(self.position);

        [
            p.and_then(Point::right),
            p.and_then(Point::down),
            p.and_then(Point::down).and_then(Point::right),
            p.and_then(Point::down)
                .and_then(Point::right)
                .and_then(Point::right),
            p.and_then(Point::down)
                .and_then(Point::down)
                .and_then(Point::right),
        ]
        .into_iter()
        .collect()
    }

    fn ell_points(&self) -> Option<Vec<Point>> {
        let p = Some(self.position);

        [
            p.and_then(Point::right).and_then(Point::right),
            p.and_then(Point::down)
                .and_then(Point::right)
                .and_then(Point::right),
            p.and_then(Point::down).and_then(Point::down),
            p.and_then(Point::down)
                .and_then(Point::down)
                .and_then(Point::right),
            p.and_then(Point::down)
                .and_then(Point::down)
                .and_then(Point::right)
                .and_then(Point::right),
        ]
        .into_iter()
        .collect()
    }

    fn column_points(&self) -> Option<Vec<Point>> {
        let p = Some(self.position);

        [
            p,
            p.and_then(Point::down),
            p.and_then(Point::down).and_then(Point::down),
            p.and_then(Point::down)
                .and_then(Point::down)
                .and_then(Point::down),
        ]
        .into_iter()
        .collect()
    }

    fn square_points(&self) -> Option<Vec<Point>> {
        let p = Some(self.position);

        [
            p,
            p.and_then(Point::right),
            p.and_then(Point::down),
            p.and_then(Point::down).and_then(Point::right),
        ]
        .into_iter()
        .collect()
    }

    pub fn legal(&self) -> bool {
        self.points().is_some()
    }
}

type Row = [Tile; WIDTH];

struct Tower {
    grid: Vec<Row>,
    top: usize,
}

impl Tower {
    fn new() -> Self {
        Self {
            grid: vec![[Air; WIDTH]; 7],
            top: 0,
        }
    }

    fn recalculate_top(&mut self) {
        let old_top = self.top;
        self.top = self.find_top(old_top);
    }

    fn find_top(&self, hint: usize) -> usize {
        self.grid
            .iter()
            .enumerate()
            .skip(hint)
            .find(|(_i, row)| row.iter().all(|tile| *tile == Air))
            .map(|(i, _row)| i)
            .unwrap()
    }

    fn draw_piece(&mut self, piece: Piece) {
        for point in piece.points().unwrap() {
            self.grid[point.y][point.x] = Rock;
        }
    }

    #[allow(unused)]
    fn erase_piece(&mut self, piece: Piece) {
        for point in piece.points().unwrap() {
            self.grid[point.y][point.x] = Air;
        }
    }

    fn lock_in_piece(&mut self, piece: Piece) -> Option<usize> {
        self.draw_piece(piece);

        // TODO: inefficient
        for _ in 0..piece.kind.height() {
            self.grid.push(Default::default());
        }

        for row in piece.position.y - 2..=piece.position.y + 2 {
            if self.grid[row].into_iter().all(|tile| tile == Rock) {
                return Some(row);
            }
        }

        None
    }

    #[allow(unused)]
    fn print(&self) {
        for row in self.grid.iter().rev() {
            let row: String = row.iter().copied().map(char::from).collect();
            println!("|{row}|");
        }

        println!("+-------+");
        println!();
    }

    fn lookup(&self, point: Point) -> Tile {
        self.grid[point.y][point.x]
    }

    fn collides(&self, piece: Piece) -> bool {
        let Some(points) = piece.points() else {
            return false
        };

        for point in points {
            if self.lookup(point) == Rock {
                return true;
            }
        }

        false
    }

    pub fn top_is_floor(&self) -> bool {
        self.grid[self.top].into_iter().all(|tile| tile == Rock)
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut tower = Tower::new();
    let mut jets = input.trim_end().chars().map(Direction::from).cycle();
    let mut piece_kinds = [Line, Plus, Ell, Column, Square].into_iter().cycle();
    let mut rocks_fallen = 0;

    #[derive(Debug)]
    enum MyOption {
        None1,
        None2,
        Some(usize),
    }

    let mut rocks_before_loop = MyOption::None1;
    let mut height_before_loop = 0;
    let mut cycles = 0;
    let mut rocks_per_loop = None;
    let mut height_per_loop = 0;

    while rocks_fallen < 5_022 {
        let piece_kind = piece_kinds.next().unwrap();
        let mut piece = Piece::new(piece_kind, tower.top);

        // Until piece rests
        loop {
            let jet_direction = jets.next().unwrap();
            let _shifted = piece.shift(jet_direction, &tower);
            let descended = piece.descend(&tower);

            if descended.is_none() {
                break;
            }
        }

        let completed_row = tower.lock_in_piece(piece);
        tower.recalculate_top();
        rocks_fallen += 1;

        if let Some(_completed_row) = completed_row {
            // dbg!(completed_row);
            // dbg!(rocks_fallen);
            // dbg!(tower.top);
            // println!();

            match rocks_before_loop {
                MyOption::None1 => rocks_before_loop = MyOption::None2,
                MyOption::None2 => {
                    rocks_before_loop = MyOption::Some(rocks_fallen);
                    height_before_loop = tower.top;
                }
                MyOption::Some(rocks_before_loop) => {
                    cycles += 1;
                    if cycles == 18 {
                        rocks_per_loop = Some(rocks_fallen - rocks_before_loop);
                        height_per_loop = tower.top - height_before_loop;
                        break;
                    }
                }
            };

            // tower.print();
        }
    }

    println!("part1 = {}", tower.top);
    // tower.print();

    // for (i, row) in tower.grid.into_iter().enumerate() {
    //     if row.into_iter().all(|tile| tile == Rock) {
    //         dbg!(i);
    //     }
    // }

    let MyOption::Some(rocks_before_loop) = rocks_before_loop else {
        panic!();
    };
    let height_before_loop = height_before_loop;
    let rocks_per_loop = rocks_per_loop.unwrap();

    dbg!(height_before_loop);
    dbg!(rocks_before_loop);
    println!();
    dbg!(rocks_per_loop);
    dbg!(height_per_loop);

    let remaining_rocks = 1_000_000_000_000 - rocks_before_loop;
    let loops = remaining_rocks / rocks_per_loop;
    let height = height_before_loop + height_per_loop * loops;

    let mut remaining_rocks = remaining_rocks - loops * rocks_per_loop;

    println!();
    dbg!(remaining_rocks);
    dbg!(loops);
    dbg!(height);
    let tower_top_before_final_loops = tower.top;

    while remaining_rocks != 0 {
        (piece_kinds, jets) = drop_rock(&mut tower, piece_kinds, jets);
        remaining_rocks -= 1;
    }

    let height = height + tower.top - tower_top_before_final_loops;
    dbg!(height);
}

fn drop_rock<I: Iterator<Item = PieceKind>, J: Iterator<Item = Direction>>(
    tower: &mut Tower,
    mut piece_kinds: I,
    mut jets: J,
) -> (I, J) {
    let piece_kind = piece_kinds.next().unwrap();
    let mut piece = Piece::new(piece_kind, tower.top);

    // Until piece rests
    loop {
        let jet_direction = jets.next().unwrap();
        let _shifted = piece.shift(jet_direction, tower);
        let descended = piece.descend(tower);

        if descended.is_none() {
            break;
        }
    }

    let _completed_row = tower.lock_in_piece(piece);
    tower.recalculate_top();

    (piece_kinds, jets)
}
