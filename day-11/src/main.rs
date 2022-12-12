use std::num::ParseIntError;
use std::str::FromStr;

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

impl<'a, T: Iterator<Item = &'a str>> From<T> for Test {
    fn from(input: T) -> Self {
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
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().skip(1);

        let (_rest, items) = lines.next().unwrap().split_once(": ").unwrap();
        let items = items.split(", ").map(|n| n.parse().unwrap()).collect();

        let (_rest, operation) = lines.next().unwrap().split_once(" = ").unwrap();
        let operation = operation.into();

        let test = Test::from(lines);

        Self {
            items,
            operation,
            test,
        }
    }
}

impl Monkey {
    fn turn<P: Relief>(&mut self, big_boy: u64) -> Vec<(usize, Item)> {
        let items = std::mem::take(&mut self.items);

        items
            .into_iter()
            .map(|item| self.operation.apply(item))
            .map(|item| P::relief(item, big_boy))
            .map(|item| (self.test.which_monkey(&item), item))
            .collect()
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let mut part1_monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();
    let mut part1_counts = vec![0; part1_monkeys.len()];
    let mut part2_monkeys = part1_monkeys.clone();
    let mut part2_counts = part1_counts.clone();

    let big_boy: u64 = part1_monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    for _ in 0..20 {
        for m in 0..part1_monkeys.len() {
            let moves: Vec<_> = part1_monkeys[m].turn::<Part1>(big_boy);

            part1_counts[m] += moves.len();

            for (to, item) in moves {
                part1_monkeys[to].items.push(item);
            }
        }
    }

    part1_counts.sort();
    let part1: usize = part1_counts.into_iter().rev().take(2).product();
    println!("part1 = {part1}");

    for _ in 0..10_000 {
        for m in 0..part2_monkeys.len() {
            let moves: Vec<_> = part2_monkeys[m].turn::<Part2>(big_boy);

            part2_counts[m] += moves.len();

            for (to, item) in moves {
                part2_monkeys[to].items.push(item);
            }
        }
    }

    part2_counts.sort();
    let part2: usize = part2_counts.into_iter().rev().take(2).product();
    println!("part2 = {part2}");
}

trait Relief {
    fn relief(item: Item, big_boy: u64) -> Item;
}

struct Part1;
struct Part2;

impl Relief for Part1 {
    fn relief(item: Item, _big_boy: u64) -> Item {
        Item(item.0 / 3)
    }
}

impl Relief for Part2 {
    fn relief(item: Item, big_boy: u64) -> Item {
        Item(item.0 % big_boy)
    }
}
