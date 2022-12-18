use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    z: isize,
    y: isize,
    x: isize,
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let (z, rest) = input.split_once(',').unwrap();
        let (y, x) = rest.split_once(',').unwrap();

        let z = z.parse().unwrap();
        let y = y.parse().unwrap();
        let x = x.parse().unwrap();

        Self { z, y, x }
    }
}

impl Point {
    fn neighbors(&self) -> [Self; 6] {
        [
            self.up(),
            self.down(),
            self.left(),
            self.right(),
            self.in_(),
            self.out(),
        ]
    }

    fn up(self) -> Self {
        Self {
            y: self.y + 1,
            ..self
        }
    }

    fn down(self) -> Self {
        Self {
            y: self.y - 1,
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

    fn in_(self) -> Self {
        Self {
            z: self.z - 1,
            ..self
        }
    }

    fn out(self) -> Self {
        Self {
            z: self.z + 1,
            ..self
        }
    }

    fn escaped(&self) -> bool {
        self.z > 33 || self.z < -10 || self.y > 33 || self.y < -10 || self.x > 33 || self.x < -10
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let points: HashSet<Point> = input.lines().map(Point::from).collect();

    let mut grid = vec![vec![vec![false; 33]; 33]; 33];

    part1(&mut grid, &points);
    part2(&points);
}

fn part1(grid: &mut [Vec<Vec<bool>>], points: &HashSet<Point>) {
    let mut surface_area = 0;
    for point in points {
        *lookup_mut(grid, *point) = true;
        surface_area += 6;

        for neighbor in point.neighbors() {
            if lookup(grid, neighbor) {
                surface_area -= 2;
            }
        }
    }

    println!("part1 = {surface_area}");
}

fn part2(points: &HashSet<Point>) {
    let mut queue = VecDeque::from([Point {
        z: 22,
        y: 22,
        x: 22,
    }]);
    let mut outside: HashSet<Point> = HashSet::from_iter(queue.iter().copied());

    while let Some(point) = queue.pop_front() {
        for neighbor in point.neighbors() {
            if outside.contains(&neighbor) || neighbor.escaped() || points.contains(&neighbor) {
                continue;
            }

            outside.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    let mut surface_area = 0;
    for point in points {
        for neighbor in point.neighbors() {
            if outside.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }

    println!("part2 = {surface_area}")
}

fn lookup(grid: &[Vec<Vec<bool>>], point: Point) -> bool {
    grid[(point.z + 10) as usize][(point.y + 10) as usize][(point.x + 10) as usize]
}

fn lookup_mut(grid: &mut [Vec<Vec<bool>>], point: Point) -> &mut bool {
    &mut grid[point.z as usize + 10][point.y as usize + 10][point.x as usize + 10]
}
