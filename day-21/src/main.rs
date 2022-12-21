use std::collections::HashMap;

#[derive(Debug)]
enum Monkey<'a> {
    Number(u64),
    Math {
        operation: Operation,
        dependents: (&'a str, &'a str),
    },
}

impl<'a> From<&'a str> for Monkey<'a> {
    fn from(value: &'a str) -> Self {
        if value.chars().next().unwrap().is_ascii_digit() {
            Self::Number(value.parse().unwrap())
        } else {
            let mut iter = value.split(' ');
            let a = iter.next().unwrap();
            let operation = iter.next().unwrap();
            let b = iter.next().unwrap();

            let operation = operation.into();
            let dependents = (a, b);

            Self::Math {
                operation,
                dependents,
            }
        }
    }
}

fn parse_monkey(input: &str) -> (&str, Monkey) {
    let (name, rest) = input.split_once(": ").unwrap();
    let monkey = Monkey::from(rest);

    (name, monkey)
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!("bad operation: {value}"),
        }
    }
}

impl Operation {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let monkeys: HashMap<&str, Monkey> = input.lines().map(parse_monkey).collect();
    dbg!(eval_monkey("root", &monkeys));
}

fn eval_monkey(monkey: &str, monkeys: &HashMap<&str, Monkey>) -> u64 {
    match monkeys.get(monkey).unwrap() {
        Monkey::Number(number) => *number,
        Monkey::Math {
            operation,
            dependents,
        } => {
            let (a, b) = dependents;
            let a = eval_monkey(a, monkeys);
            let b = eval_monkey(b, monkeys);
            operation.apply(a, b)
        }
    }
}
