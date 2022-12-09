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
        let x_diff = (head.x - self.x).abs();
        let y_diff = (head.y - self.y).abs();

        if x_diff <= 1 && y_diff <= 1 {
            // Don't do anything if the tail is already adjacent
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
}

fn main() {
    let input = match std::env::var("TEST") {
        Ok(number) => read_to_string(format!("test_input{number}.txt")),
        Err(_) => read_to_string("input.txt"),
    }
    .unwrap();

    let mut part1_head = Point::default();
    let mut part1_tail = Point::default();
    let mut part1_seen: HashSet<Point> = [part1_tail].into();

    let mut part2_head = Point::default();
    let mut tail_1 = Point::default();
    let mut tail_2 = Point::default();
    let mut tail_3 = Point::default();
    let mut tail_4 = Point::default();
    let mut tail_5 = Point::default();
    let mut tail_6 = Point::default();
    let mut tail_7 = Point::default();
    let mut tail_8 = Point::default();
    let mut tail_9 = Point::default();
    let mut part2_seen: HashSet<Point> = [tail_9].into();

    for motion in input.lines().map(Motion::from) {
        // Part 1
        for head in part1_head.apply(motion) {
            part1_tail.follow(head);
            part1_seen.insert(part1_tail);
        }

        // Part 2
        for head in part2_head.apply(motion) {
            tail_1.follow(head);
            tail_2.follow(tail_1);
            tail_3.follow(tail_2);
            tail_4.follow(tail_3);
            tail_5.follow(tail_4);
            tail_6.follow(tail_5);
            tail_7.follow(tail_6);
            tail_8.follow(tail_7);
            tail_9.follow(tail_8);
            part2_seen.insert(tail_9);
        }
    }

    println!("part1 = {}", part1_seen.len());
    println!("part2 = {}", part2_seen.len());
}
