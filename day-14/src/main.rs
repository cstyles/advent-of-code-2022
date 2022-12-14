use std::cmp::Ordering;
use std::fmt::Display;

use Tile::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> Self {
        match tile {
            Air => '.',
            Rock => '#',
            Sand => 'o',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    y: usize,
    x: usize,
}

impl From<&str> for Point {
    fn from(pair: &str) -> Self {
        let (x, y) = pair.split_once(',').unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Self { y, x }
    }
}

impl Point {
    fn between(&self, other: &Self) -> Vec<Self> {
        match (self.x.cmp(&other.x), self.y.cmp(&other.y)) {
            (Ordering::Equal, Ordering::Less) => self.y_range_positive(other.y),
            (Ordering::Equal, _) => self.y_range_negative(other.y),
            (Ordering::Less, _) => self.x_range_positive(other.x),
            (Ordering::Greater, _) => self.x_range_negative(other.x),
        }
    }

    fn y_range_positive(&self, other_y: usize) -> Vec<Self> {
        (self.y..other_y).map(|y| Self { y, x: self.x }).collect()
    }

    fn y_range_negative(&self, other_y: usize) -> Vec<Self> {
        (other_y + 1..=self.y)
            .map(|y| Self { y, x: self.x })
            .collect()
    }

    fn x_range_positive(&self, other_x: usize) -> Vec<Self> {
        (self.x..other_x).map(|x| Self { x, y: self.y }).collect()
    }

    fn x_range_negative(&self, other_x: usize) -> Vec<Self> {
        (other_x + 1..=self.x)
            .map(|x| Point { x, y: self.y })
            .collect()
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

    let mut grid = vec![vec![Air; 600]; 200];
    let paths = input
        .lines()
        .map(|line| line.split(" -> ").map(Point::from));

    for path in paths {
        draw_rocks(&mut grid, path);
    }

    let abyss = find_lowest_rock(&grid);

    let mut i = 0;
    while drop_sand(&mut grid, abyss).is_some() {
        i += 1;
    }

    println!("part1 = {i}");
}

fn draw_rocks(grid: &mut [Vec<Tile>], mut path: impl Iterator<Item = Point>) {
    let mut start = path.next().unwrap();

    for end in path {
        for point in start.between(&end) {
            draw_tile(grid, point, Rock);
        }

        start = end;
    }

    draw_tile(grid, start, Rock);
}

fn draw_tile(grid: &mut [Vec<Tile>], point: Point, tile: Tile) {
    grid[point.y][point.x] = tile;
}

#[allow(unused)]
fn print_grid(grid: &[Vec<Tile>]) {
    for row in grid {
        let row: String = row.iter().map(char::from).collect();
        println!("{row}");
    }
}

fn drop_sand(grid: &mut [Vec<Tile>], abyss: usize) -> Option<Point> {
    find_resting_place(grid, abyss).tap(|point| grid[point.y][point.x] = Sand)
}

fn lookup(grid: &[Vec<Tile>], point: Point) -> Tile {
    grid[point.y][point.x]
}

fn find_resting_place(grid: &[Vec<Tile>], abyss: usize) -> Option<Point> {
    let mut point = Point { x: 500, y: 0 };

    loop {
        if point.y == abyss {
            return None;
        }

        let down = point.down();
        if lookup(grid, down) == Air {
            point = down;
            continue;
        }

        let down_left = point.down().left();
        if lookup(grid, down_left) == Air {
            point = down_left;
            continue;
        }

        let down_right = point.down().right();
        if lookup(grid, down_right) == Air {
            point = down_right;
            continue;
        }

        return Some(point);
    }
}

// Anything below the lowest rock will fall into the abyss
fn find_lowest_rock(grid: &[Vec<Tile>]) -> usize {
    let position_from_bottom = grid
        .iter()
        .rev()
        .position(|row| row.contains(&Rock))
        .unwrap();

    grid.len() - position_from_bottom - 1
}

trait OptionExt<T> {
    fn tap(self, f: impl FnOnce(&T)) -> Self;
}

impl<T> OptionExt<T> for Option<T> {
    fn tap(self, f: impl FnOnce(&T)) -> Self {
        if let Some(inner) = self.as_ref() {
            f(inner);
        }

        self
    }
}
