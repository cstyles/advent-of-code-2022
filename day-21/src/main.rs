use std::collections::HashMap;

#[derive(Debug)]
enum Monkey<'a> {
    Number(u64),
    Math {
        operation: Operation,
        dependents: (&'a str, &'a str),
    },
    Human(u64),
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
    if name == "humn" {
        (name, Monkey::Human(rest.parse().unwrap()))
    } else {
        let monkey = Monkey::from(rest);
        (name, monkey)
    }
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

    fn to_char(self) -> char {
        match self {
            Operation::Add => '+',
            Operation::Subtract => '-',
            Operation::Multiply => '*',
            Operation::Divide => '/',
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
    println!("part1 = {}", eval_monkey("root", &monkeys));

    let Monkey::Math { dependents: (a, b), .. } = monkeys.get("root").unwrap() else { unreachable!() };

    let Term::Str(a) = generate_term(a, &monkeys) else { unreachable!() };
    let Term::Num(b) = generate_term(b, &monkeys) else { unreachable!() };

    println!("{a}");
    println!("{b}");

    // I did the reduction by hand
    println!("part2 = 3032671800353");
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
        Monkey::Human(number) => *number, // part1
    }
}

#[derive(Debug)]
enum Term {
    Str(String),
    Num(u64),
}

fn generate_term(monkey: &str, monkeys: &HashMap<&str, Monkey>) -> Term {
    match monkeys.get(monkey).unwrap() {
        Monkey::Number(number) => Term::Num(*number),
        Monkey::Math {
            operation,
            dependents,
        } => {
            let (a, b) = dependents;
            let a = generate_term(a, monkeys);
            let b = generate_term(b, monkeys);

            match (a, b) {
                (Term::Str(a), Term::Num(b)) => {
                    let operation = operation.to_char();
                    Term::Str(format!("({a} {operation} {b})"))
                }
                (Term::Num(a), Term::Str(b)) => {
                    let operation = operation.to_char();
                    Term::Str(format!("({a} {operation} {b})"))
                }
                (Term::Num(a), Term::Num(b)) => Term::Num(operation.apply(a, b)),
                (Term::Str(_), Term::Str(_)) => unreachable!(),
            }
        }
        Monkey::Human(_number) => Term::Str("human".into()),
    }
}
