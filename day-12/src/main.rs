use std::collections::BinaryHeap;
use std::ops::Add;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(y: usize, x: usize) -> Self {
        Self { x, y }
    }

    fn in_bounds(&self, grid: &[Vec<i8>]) -> bool {
        let rows = grid.len();
        let columns = grid[0].len();

        self.y < rows && self.x < columns
    }

    fn neighbors(self, grid: &[Vec<i8>]) -> impl Iterator<Item = Point> + '_ {
        [
            self.y.checked_sub(1).map(|y| Point::new(y, self.x)),
            Some(self + Point::new(1, 0)),
            self.x.checked_sub(1).map(|x| Point::new(self.y, x)),
            Some(self + Point::new(0, 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|point| point.in_bounds(grid))
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
struct Node {
    point: Point,
    distance: u32,
}

impl Ord for Node {
    // Reverse ordering to create a min heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(height).collect::<Vec<_>>())
        .collect();

    let start = find(0, &grid);
    let end = find(27, &grid);

    let distances = dijkstra(start, end, &grid);

    println!("part1 = {}", distances[start.y][start.x]);

    let mut starting_points = find_all(1, &grid);
    starting_points.push(start);
    let part2 = starting_points
        .into_iter()
        .map(|point| lookup(point, &distances))
        .min()
        .unwrap();
    println!("part2 = {part2}");
}

// Find the distances from the *end* to the *start*
fn dijkstra(start: Point, end: Point, grid: &[Vec<i8>]) -> Vec<Vec<u32>> {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut heap: BinaryHeap<Node> = BinaryHeap::with_capacity(rows * columns);
    let mut distances = vec![vec![u32::MAX; columns]; rows];
    distances[end.y][end.x] = 0;

    // Set distance of "start" node to 0
    heap.push(Node {
        point: end,
        distance: 0,
    });

    while let Some(current) = heap.pop() {
        let current_height = lookup(current.point, grid);

        for neighbor_point in current.point.neighbors(grid) {
            let neighbor_height = lookup(neighbor_point, grid);
            if neighbor_height - current_height < -1 {
                // Too tall to climb from here
                continue;
            }

            let existing_distance = lookup(neighbor_point, &distances);
            let neighbor_node = Node {
                point: neighbor_point,
                distance: current.distance + 1,
            };

            if neighbor_node.distance < *existing_distance {
                *lookup_mut(neighbor_point, &mut distances) = neighbor_node.distance;
            } else {
                continue;
            }

            heap.push(neighbor_node);

            if neighbor_point == start {
                return distances;
            }
        }
    }

    distances
}

fn height(c: char) -> i8 {
    match c {
        'S' => 0,
        'E' => 27,
        c => c as i8 - 96,
    }
}

fn find(target: i8, grid: &[Vec<i8>]) -> Point {
    for (row_number, row) in grid.iter().enumerate() {
        for (column_number, column) in row.iter().enumerate() {
            if *column == target {
                return Point::new(row_number, column_number);
            }
        }
    }

    unreachable!("target ({}) not found", target);
}

fn find_all(target: i8, grid: &[Vec<i8>]) -> Vec<Point> {
    let mut points = vec![];

    for (row_number, row) in grid.iter().enumerate() {
        for (column_number, column) in row.iter().enumerate() {
            if *column == target {
                points.push(Point::new(row_number, column_number));
            }
        }
    }

    points
}

fn lookup<T>(point: Point, grid: &[Vec<T>]) -> &T {
    &grid[point.y][point.x]
}

fn lookup_mut<T>(point: Point, grid: &mut [Vec<T>]) -> &mut T {
    &mut grid[point.y][point.x]
}

#[allow(unused)]
fn print_grid(grid: &[Vec<u32>]) {
    for row in grid {
        for cell in row {
            print!("{cell:02}, ");
        }
        println!();
    }
}
