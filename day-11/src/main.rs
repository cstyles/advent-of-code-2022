use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::{FromStr, Lines};

#[derive(Debug, Copy, Clone)]
struct Operation {
    op: Op,
    term: Term,
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let (_old, rest) = line.split_once(' ').unwrap();
        let (op, term) = rest.split_once(' ').unwrap();

        let term = Term::from(term);
        let op = Op::from(op);

        Self { op, term }
    }
}

impl Operation {
    fn apply(&self, item: Item) -> Item {
        match (self.op, self.term) {
            (Op::Add, Term::Literal(literal)) => Item(item.0 + literal),
            (Op::Add, Term::Old) => Item(item.0 + item.0),
            (Op::Mul, Term::Literal(literal)) => Item(item.0 * literal),
            (Op::Mul, Term::Old) => Item(item.0 * item.0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => unreachable!("bad op: {s}"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Term {
    Literal(u64),
    Old,
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        match s {
            "old" => Self::Old,
            x => Self::Literal(x.parse().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divisible_by: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl From<Lines<'_>> for Test {
    fn from(input: Lines) -> Self {
        let mut lines = input.map(get_number_at_end);
        let divisible_by = lines.next().unwrap();
        let true_monkey = lines.next().unwrap() as usize;
        let false_monkey = lines.next().unwrap() as usize;

        Self {
            divisible_by,
            true_monkey,
            false_monkey,
        }
    }
}

impl Test {
    fn which_monkey(&self, item: &Item) -> usize {
        if item.0 % self.divisible_by == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

fn get_number_at_end(string: &str) -> u64 {
    string.rsplit_once(' ').unwrap().1.parse().unwrap()
}

#[derive(Debug, Copy, Clone)]
struct Item(u64);

impl FromStr for Item {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    number: u64,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let number: u64 = lines
            .next()
            .unwrap()
            .chars()
            .nth(7)
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;

        let (_rest, items) = lines.next().unwrap().split_once(": ").unwrap();
        let items = items.split(", ").map(|n| n.parse().unwrap()).collect();

        let (_rest, operation) = lines.next().unwrap().split_once(" = ").unwrap();
        let operation = operation.into();

        let test = Test::from(lines);

        Self {
            number,
            items,
            operation,
            test,
        }
    }
}

impl Monkey {
    fn turn(&mut self) -> impl Iterator<Item = (usize, Item)> + '_ {
        std::iter::from_fn(move || {
            if self.items.is_empty() {
                None
            } else {
                let item = self.items.remove(0);
                let new_item = self.operation.apply(item);
                let new_item = Item(new_item.0 / 3);
                Some((self.test.which_monkey(&new_item), new_item))
            }
        })
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let mut counts: HashMap<usize, usize> = HashMap::with_capacity(monkeys.len());

    // Rounds
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            let moves: Vec<_> = monkeys[m].turn().collect();

            counts
                .entry(m)
                .and_modify(|x| *x += moves.len())
                .or_insert(moves.len());

            for (to, item) in moves {
                monkeys[to].items.push(item);
            }
        }
    }

    let mut counts: Vec<usize> = counts.values().copied().collect();
    counts.sort();
    let part1: usize = counts.into_iter().rev().take(2).product();
    println!("part1 = {part1}");
}
