use std::collections::HashSet;

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
            self.0
                .get(row_number - 1)
                .and_then(|row| row.get(column_number))
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
