use std::collections::HashSet;
use std::iter::{Zip, Rev, Repeat};
use std::ops::{RangeInclusive, RangeFrom};

type Trees = HashSet<(usize, usize)>;

#[derive(Debug, Copy, Clone)]
enum Index {
    Row(usize),
    Column(usize),
}

struct Grid(Vec<Vec<u32>>);

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        Self(grid)
    }
}

impl Grid {
    fn get(&self, row_number: usize, column_number: usize) -> Option<&u32> {
        self.0
            .get(row_number)
            .and_then(|row| row.get(column_number))
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<u32>> {
        self.0.iter()
    }

    fn row(&self, row_number: usize) -> impl Iterator<Item = &u32> {
        self.0[row_number].iter()
    }

    fn column(&self, column_number: usize) -> Option<impl Iterator<Item = &u32>> {
        if column_number >= self.0.len() {
            return None;
        }

        let mut row_number = 0;
        Some(std::iter::from_fn(move || {
            row_number += 1;
            self.get(row_number - 1, column_number)
        }))
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &u32>> {
        let mut column_number = 0;
        std::iter::from_fn(move || {
            column_number += 1;
            self.column(column_number - 1)
        })
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let grid = Grid::from(input);
    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid) {
    let mut visible: Trees = Trees::default();

    for (row_number, row) in grid.rows().enumerate() {
        let other_coordinate = Index::Row(row_number);

        // Look from the left
        find_visible_trees(
            row.iter().copied().enumerate(),
            &mut visible,
            other_coordinate,
        );

        // Look from the right
        find_visible_trees(
            row.iter().copied().enumerate().rev(),
            &mut visible,
            other_coordinate,
        );
    }

    // Look from the top
    for (column_number, column) in grid.columns().enumerate() {
        let other_coordinate = Index::Column(column_number);
        find_visible_trees(column.copied().enumerate(), &mut visible, other_coordinate);
    }

    // Look from the bottom
    for (column_number, column) in grid.columns().enumerate() {
        let other_coordinate = Index::Column(column_number);
        let mut column: Vec<_> = column.copied().enumerate().collect();
        column.reverse();
        find_visible_trees(column, &mut visible, other_coordinate);
    }

    println!("part1 = {}", visible.len());
}

fn part2(grid: &Grid) {
    let dimension = grid.rows().count();
    let mut max_scenic_score = 0;

    // Iterate over all non-edge trees
    for (row_number, row) in grid.rows().enumerate().skip(1).take(dimension - 2) {
        for (column_number, _tree) in row.iter().enumerate().skip(1).take(dimension - 2) {
            let scenic_score = scenic_score(grid, row_number, column_number);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("part2 = {max_scenic_score}");
}

fn scenic_score(grid: &Grid, row_number: usize, column_number: usize) -> u32 {
    let left = look::<Left>(grid, row_number, column_number);
    let right = look::<Right>(grid, row_number, column_number);
    let up = look::<Up>(grid, row_number, column_number);
    let down = look::<Down>(grid, row_number, column_number);

    left * right * up * down
}

fn find_visible_trees(
    trees: impl IntoIterator<Item = (usize, u32)>,
    visible: &mut Trees,
    other_coordinate: Index,
) {
    let mut trees = trees.into_iter();
    let (i, mut tallest) = trees.next().unwrap();
    insert(i, other_coordinate, visible);

    for (i, tree) in trees {
        if tree > tallest {
            tallest = tree;
            insert(i, other_coordinate, visible);
        }
    }
}

fn insert(coordinate: usize, other_coordinate: Index, visible: &mut Trees) {
    match other_coordinate {
        Index::Row(row_number) => visible.insert((row_number, coordinate)),
        Index::Column(column_number) => visible.insert((coordinate, column_number)),
    };
}

trait Direction {
    type Range: Iterator<Item = (usize, usize)>;

    fn range(row_number: usize, column_number: usize) -> Self::Range;
}

fn look<Dir: Direction>(grid: &Grid, row_number: usize, column_number: usize) -> u32 {
    let this_tree = grid.get(row_number, column_number).unwrap();
    let mut total = 0;

    for (row_number, column_number) in Dir::range(row_number, column_number) {
        match grid.get(row_number, column_number) {
            None => break,
            Some(height) => {
                total += 1;
                if height >= this_tree {
                    break;
                }
            }
        }
    }

    total
}

struct Left;
struct Right;
struct Up;
struct Down;

impl Direction for Left {
    type Range = Zip<Repeat<usize>, Rev<RangeInclusive<usize>>>;

    fn range(row_number: usize, column_number: usize) -> Self::Range {
        std::iter::repeat(row_number).zip((0..=column_number - 1).rev())
    }
}

impl Direction for Right {
    type Range = Zip<Repeat<usize>, RangeFrom<usize>>;

    fn range(row_number: usize, column_number: usize) -> Self::Range {
        std::iter::repeat(row_number).zip((column_number + 1)..)
    }
}

impl Direction for Up {
    type Range = Zip<Rev<RangeInclusive<usize>>, Repeat<usize>>;

    fn range(row_number: usize, column_number: usize) -> Self::Range {
        (0..=row_number - 1).rev().zip(std::iter::repeat(column_number))
    }
}

impl Direction for Down {
    type Range = Zip<RangeFrom<usize>, Repeat<usize>>;

    fn range(row_number: usize, column_number: usize) -> Self::Range {
        (row_number + 1..).zip(std::iter::repeat(column_number))
    }
}
