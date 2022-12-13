use std::cmp::Ordering;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(input: &str) -> Self {
        let mut chars = input.chars().peekable();
        debug_assert_eq!(Some('['), chars.next());

        let (unparsed, packet) = parse_list(chars);
        debug_assert_eq!(0, unparsed.count());

        packet
    }
}

fn parse_list<I: Iterator<Item = char>>(mut chars: Peekable<I>) -> (Peekable<I>, Packet) {
    let mut list = vec![];

    while let Some(c) = chars.next() {
        match c {
            ',' => continue,
            ']' => break,
            '[' => {
                let result = parse_list(chars);
                chars = result.0;
                list.push(result.1);
            }
            c => {
                let first_digit = c.to_digit(10).unwrap();
                let number = if chars.peek().unwrap().is_ascii_digit() {
                    let second_digit = chars.next().unwrap().to_digit(10).unwrap();
                    first_digit * 10 + second_digit
                } else {
                    first_digit
                };

                let packet = Packet::Number(number);
                list.push(packet);
            }
        }
    }

    (chars, Packet::List(list))
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
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
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

    let mut all_packets: Vec<_> = pairs
        .clone()
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect();

    let two = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    all_packets.push(two.clone());
    all_packets.push(six.clone());

    let orderings = pairs.into_iter().map(|(a, b)| a.cmp(&b));

    let part1: u32 = (1..)
        .zip(orderings)
        .filter(|(_, ordering)| *ordering == Ordering::Less)
        .map(|(i, _)| i)
        .sum();

    println!("part1 = {part1}");

    all_packets.sort();

    let two_index = all_packets
        .iter()
        .position(|packet| *packet == two)
        .unwrap()
        + 1;
    let six_index = all_packets
        .iter()
        .position(|packet| *packet == six)
        .unwrap()
        + 1;

    println!("part2 = {}", two_index * six_index);
}
