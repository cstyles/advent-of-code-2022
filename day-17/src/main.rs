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

    fn lock_in_piece(&mut self, piece: Piece) {
        self.draw_piece(piece);

        // TODO: inefficient
        for _ in 0..piece.kind.height() {
            self.grid.push(Default::default());
        }
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
    // println!("Start:");
    // tower.print();

    while rocks_fallen < 2022 {
        // std::thread::sleep(std::time::Duration::from_millis(50));

        let piece_kind = piece_kinds.next().unwrap();
        let mut piece = Piece::new(piece_kind, tower.top);
        // println!("New piece:");

        // Until piece rests
        loop {
            // tower.draw_piece(piece);
            // tower.print();
            // tower.erase_piece(piece);

            let jet_direction = jets.next().unwrap();
            // println!("trying to move {jet_direction:?}");
            let shifted = piece.shift(jet_direction, &tower);

            // if shifted.is_some() {
            //     println!("moved {jet_direction:?}");
            // } else {
            //     println!("didn't move horizontally");
            // }

            // tower.draw_piece(piece);
            // tower.print();
            // tower.erase_piece(piece);

            // println!("Descending");
            let descended = piece.descend(&tower);

            if descended.is_none() {
                break;
            }
        }

        // dbg!(piece);
        tower.lock_in_piece(piece);
        tower.recalculate_top();
        rocks_fallen += 1;
        // tower.print();
    }

    // tower.print();
    println!("part1 = {}", tower.top);
}
