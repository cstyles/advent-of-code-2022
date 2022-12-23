use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

use Direction::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    West,
    East,
    SouthWest,
    South,
    SouthEast,
}

const NORTH_SET: ([Direction; 3], Direction) = ([NorthWest, North, NorthEast], North);
const SOUTH_SET: ([Direction; 3], Direction) = ([SouthWest, South, SouthEast], South);
const WEST_SET: ([Direction; 3], Direction) = ([NorthWest, West, SouthWest], West);
const EAST_SET: ([Direction; 3], Direction) = ([NorthEast, East, SouthEast], East);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    y: isize,
    x: isize,
}

impl Point {
    fn step(&self, direction: Direction) -> Self {
        match direction {
            NorthWest => Self {
                y: self.y - 1,
                x: self.x - 1,
            },
            North => Self {
                y: self.y - 1,
                x: self.x,
            },
            NorthEast => Self {
                y: self.y - 1,
                x: self.x + 1,
            },
            West => Self {
                y: self.y,
                x: self.x - 1,
            },
            East => Self {
                y: self.y,
                x: self.x + 1,
            },
            SouthWest => Self {
                y: self.y + 1,
                x: self.x - 1,
            },
            South => Self {
                y: self.y + 1,
                x: self.x,
            },
            SouthEast => Self {
                y: self.y + 1,
                x: self.x + 1,
            },
        }
    }

    fn neighbors(&self) -> [Self; 8] {
        [
            self.step(NorthWest),
            self.step(North),
            self.step(NorthEast),
            self.step(West),
            self.step(East),
            self.step(SouthWest),
            self.step(South),
            self.step(SouthEast),
        ]
    }
}

type DirectionToConsider = ([Direction; 3], Direction);
type DirectionsToConsider = [DirectionToConsider; 4];

fn main() {
    let input = match std::env::var("TEST") {
        Ok(number) => read_to_string(format!("test_input{number}.txt")),
        Err(_) => read_to_string("input.txt"),
    }
    .unwrap();

    let mut directions_to_consider = [
        [NORTH_SET, SOUTH_SET, WEST_SET, EAST_SET],
        [SOUTH_SET, WEST_SET, EAST_SET, NORTH_SET],
        [WEST_SET, EAST_SET, NORTH_SET, SOUTH_SET],
        [EAST_SET, NORTH_SET, SOUTH_SET, WEST_SET],
    ]
    .into_iter()
    .cycle();

    let mut elves = parse_input(&input);
    for _ in 1..=10 {
        elves = round(&elves, directions_to_consider.next().unwrap());
    }

    let (top_left, bottom_right) = smallest_rectangle(&elves);
    println!("part1 = {}", area(top_left, bottom_right) - elves.len());
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut elves: HashSet<Point> = HashSet::default();
    for (y, row) in input.lines().enumerate() {
        for (x, _c) in row.chars().enumerate().filter(|(_, c)| *c == '#') {
            let (y, x) = (y as isize, x as isize);
            elves.insert(Point { y, x });
        }
    }

    elves
}

fn round(elves: &HashSet<Point>, directions_to_consider: DirectionsToConsider) -> HashSet<Point> {
    let (how_many_elves_per_destination, destinations) = propose(elves, directions_to_consider);
    act(how_many_elves_per_destination, destinations)
}

fn propose(
    elves: &HashSet<Point>,
    directions_to_consider: DirectionsToConsider,
) -> (HashMap<Point, usize>, HashMap<Point, Point>) {
    let mut how_many_elves_per_destination: HashMap<Point, usize> = HashMap::default();
    let mut destinations: HashMap<Point, Point> = HashMap::with_capacity(elves.len());

    'elves: for elf in elves {
        if stranded(elf, elves) {
            // No neighbors were around, stay still
            destinations.insert(*elf, *elf);
            continue;
        }

        for (direction_to_consider, direction_to_move) in directions_to_consider {
            let any_elves_in_direction = direction_to_consider.into_iter().any(|direction| {
                let destination = elf.step(direction);
                elves.contains(&destination)
            });

            if !any_elves_in_direction {
                let destination = elf.step(direction_to_move);
                how_many_elves_per_destination
                    .entry(destination)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                destinations.insert(*elf, destination);
                continue 'elves;
            }
        }

        // Couldn't move in any direction, stay still
        destinations.insert(*elf, *elf);
    }

    (how_many_elves_per_destination, destinations)
}

fn stranded(elf: &Point, elves: &HashSet<Point>) -> bool {
    elf.neighbors()
        .into_iter()
        .all(|neighbor| !elves.contains(&neighbor))
}

fn act(
    how_many_elves_per_destination: HashMap<Point, usize>,
    destinations: HashMap<Point, Point>,
) -> HashSet<Point> {
    destinations
        .into_iter()
        .map(|(elf, new_elf)| {
            if how_many_elves_per_destination.get(&new_elf).unwrap_or(&0) > &1 {
                elf
            } else {
                new_elf
            }
        })
        .collect()
}

fn smallest_rectangle(elves: &HashSet<Point>) -> (Point, Point) {
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;

    for elf in elves {
        max_x = elf.x.max(max_x);
        max_y = elf.y.max(max_y);
        min_x = elf.x.min(min_x);
        min_y = elf.y.min(min_y);
    }

    let top_left = Point { y: min_y, x: min_x };
    let bottom_right = Point { y: max_y, x: max_x };

    (top_left, bottom_right)
}

fn area(top_left: Point, bottom_right: Point) -> usize {
    let width = bottom_right.x - top_left.x + 1;
    let height = bottom_right.y - top_left.y + 1;

    (width * height) as usize
}
