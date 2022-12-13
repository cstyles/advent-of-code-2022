use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        let (open_bracket, input) = input.split_at(1);
        debug_assert_eq!("[", open_bracket);

        let (unparsed, packet) = parse_list(input);
        debug_assert!(unparsed.is_empty());

        packet
    }
}

fn parse_number(c: &str) -> Packet {
    Packet::Number(c.parse().unwrap())
}

fn parse_list(input: &str) -> (&str, Packet) {
    let mut list = vec![];
    let (mut _first, mut rest) = ("", input);

    while !rest.is_empty() {
        let split = rest.split_at(1);
        let first = split.0;
        rest = split.1;

        match first {
            "," => continue,
            "]" => break,
            "[" => {
                let result = parse_list(rest);
                rest = result.0;
                list.push(result.1);
            }
            s => {
                let next = rest.chars().next().unwrap();
                match next.to_digit(10) {
                    None => list.push(parse_number(s)),
                    Some(digit) => {
                        // Combine the two digits into a single number
                        let tens: u32 = s.parse().unwrap();
                        let num = tens * 10 + digit;
                        let packet = Packet::Number(num);
                        list.push(packet);
                    }
                }
            }
        }
    }

    (rest, Packet::List(list))
}

impl Packet {
    fn wrap_in_list(&self) -> Self {
        Self::List(vec![self.clone()])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::Number(_), Packet::List(_)) => self.wrap_in_list().cmp(other),
            (Packet::List(_), Packet::Number(_)) => self.cmp(&other.wrap_in_list()),
            (Packet::List(left), Packet::List(right)) => {
                let mut left = left.iter();
                let mut right = right.iter();

                loop {
                    match (left.next(), right.next()) {
                        (None, None) => return Ordering::Equal,      // Equal lengths
                        (None, Some(_)) => return Ordering::Less,    // Left ran out first
                        (Some(_), None) => return Ordering::Greater, // Right ran out first
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            ordering => return ordering,
                        },
                    }
                }
            }
        }
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let pairs: Vec<_> = input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            (lines.next().unwrap(), lines.next().unwrap())
        })
        .map(|(a, b)| (Packet::from(a), Packet::from(b)))
        .collect();

    let orderings = pairs.into_iter().map(|(a, b)| a.cmp(&b));

    let part1: u32 = (1..)
        .zip(orderings)
        .filter(|(_, ordering)| *ordering == Ordering::Less)
        .map(|(i, _)| i)
        .sum();

    println!("part1 = {part1}");
}
