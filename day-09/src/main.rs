use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::read_to_string;

use Direction::*;

#[derive(Debug, Copy, Clone)]
struct Motion {
    direction: Direction,
    steps: usize,
}

impl From<&str> for Motion {
    fn from(input: &str) -> Self {
        let (direction, steps) = input.split_once(' ').unwrap();
        let direction = direction.into();
        let steps: usize = steps.parse().unwrap();

        Self { direction, steps }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "L" => Left,
            "R" => Right,
            "U" => Up,
            "D" => Down,
            _ => panic!("invalid direction: {input}"),
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
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

    fn shift(&self, direction: Direction) -> Self {
        match direction {
            Left => self.left(),
            Right => self.right(),
            Up => self.up(),
            Down => self.down(),
        }
    }

    fn apply(&mut self, motion: Motion) -> impl Iterator<Item = Self> + '_ {
        let mut step = 0;

        std::iter::from_fn(move || {
            step += 1;
            if step > motion.steps {
                None
            } else {
                *self = self.shift(motion.direction);
                Some(*self)
            }
        })
    }

    fn follow(&mut self, head: Self) {
        // Don't do anything if the tail is already adjacent
        if self.adjacent(&head) {
            return;
        }

        *self = match (head.x.cmp(&self.x), head.y.cmp(&self.y)) {
            (Ordering::Less, Ordering::Less) => self.down().left(),
            (Ordering::Less, Ordering::Equal) => self.left(),
            (Ordering::Less, Ordering::Greater) => self.up().left(),
            (Ordering::Equal, Ordering::Less) => self.down(),
            (Ordering::Equal, Ordering::Equal) => *self,
            (Ordering::Equal, Ordering::Greater) => self.up(),
            (Ordering::Greater, Ordering::Less) => self.down().right(),
            (Ordering::Greater, Ordering::Equal) => self.right(),
            (Ordering::Greater, Ordering::Greater) => self.up().right(),
        };
    }

    fn adjacent(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

fn main() {
    let input = match std::env::var("TEST") {
        Ok(number) => read_to_string(format!("test_input{number}.txt")),
        Err(_) => read_to_string("input.txt"),
    }
    .unwrap();

    let mut head = Point::default();
    let mut tails = [Point::default(); 9];
    let mut part1_seen: HashSet<Point> = [Point::default()].into();
    let mut part2_seen: HashSet<Point> = [Point::default()].into();

    for motion in input.lines().map(Motion::from) {
        for head in head.apply(motion) {
            tails[0].follow(head);
            for i in 1..tails.len() {
                tails[i].follow(tails[i - 1]);
            }

            part1_seen.insert(tails[0]);
            part2_seen.insert(tails[8]);
        }
    }

    println!("part1 = {}", part1_seen.len());
    println!("part2 = {}", part2_seen.len());
}
