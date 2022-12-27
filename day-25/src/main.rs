use std::iter::Sum;
use std::ops::Add;

fn parse_snafu_digit(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("bad char: {c}"),
    }
}

fn parse_snafu_number(string: &str) -> i64 {
    string.chars().rev().enumerate().fold(0, |sum, (i, c)| {
        let magnitude = 5i64.pow(i as u32);
        let digit = parse_snafu_digit(c);
        let digit = digit * magnitude;
        sum + digit
    })
}

fn as_snafu_digit(num: i64) -> char {
    match num {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unreachable!(),
    }
}

fn wrap_digit(num: i64) -> i64 {
    match num {
        0 | 1 | 2 => num,
        3 | 4 => num - 5,
        _ => unreachable!(),
    }
}

fn as_snafu(mut num: i64) -> String {
    let mut string = String::new();

    while num != 0 {
        let ones = num % 5;
        let adjustment = wrap_digit(ones);
        let digit = as_snafu_digit(adjustment);
        string.push(digit);
        num -= adjustment;
        num /= 5;
    }

    string.chars().rev().collect()
}

use SnafuDigit::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum SnafuDigit {
    Zero,
    One,
    Two,
    NegOne,
    NegTwo,
}

impl Add<SnafuDigit> for SnafuDigit {
    // Returns the digit and a second digit indicating carry
    type Output = (SnafuDigit, SnafuDigit);

    fn add(self, rhs: SnafuDigit) -> Self::Output {
        match (self, rhs) {
            (Zero, _) => (rhs, Zero),
            (_, Zero) => (self, Zero),
            (One, One) => (Two, Zero),
            (One, Two) => (NegTwo, One),
            (One, NegOne) => (Zero, Zero),
            (One, NegTwo) => (NegOne, Zero),
            (Two, One) => (NegTwo, One),
            (Two, Two) => (NegOne, One),
            (Two, NegOne) => (One, Zero),
            (Two, NegTwo) => (Zero, Zero),
            (NegOne, One) => (Zero, Zero),
            (NegOne, Two) => (One, Zero),
            (NegOne, NegOne) => (NegTwo, Zero),
            (NegOne, NegTwo) => (Two, NegOne),
            (NegTwo, One) => (NegOne, Zero),
            (NegTwo, Two) => (Zero, Zero),
            (NegTwo, NegOne) => (Two, NegOne),
            (NegTwo, NegTwo) => (One, NegOne),
        }
    }
}

impl From<char> for SnafuDigit {
    fn from(c: char) -> Self {
        match c {
            '2' => Two,
            '1' => One,
            '0' => Zero,
            '-' => NegOne,
            '=' => NegTwo,
            _ => panic!("bad char: {c}"),
        }
    }
}

impl SnafuDigit {
    fn as_char(&self) -> char {
        match *self {
            NegTwo => '=',
            NegOne => '-',
            Zero => '0',
            One => '1',
            Two => '2',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Snafu {
    digits: Vec<SnafuDigit>,
}

impl<T: AsRef<str>> From<T> for Snafu {
    fn from(value: T) -> Self {
        let value = value.as_ref();
        let digits = value.chars().rev().map(SnafuDigit::from).collect();

        Self { digits }
    }
}

impl std::fmt::Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.digits.iter().rev().map(SnafuDigit::as_char).collect();
        write!(f, "{s}")
    }
}

impl Add<Snafu> for Snafu {
    type Output = Snafu;

    fn add(self, rhs: Snafu) -> Self::Output {
        let mut digits = Vec::with_capacity(self.digits.len().max(rhs.digits.len()));
        let mut i = 0;
        let mut previous_carry = Zero;

        loop {
            let (self_digit, rhs_digit) = match (self.digits.get(i), rhs.digits.get(i)) {
                (None, None) => break,
                (None, Some(rhs_digit)) => (Zero, *rhs_digit),
                (Some(self_digit), None) => (*self_digit, Zero),
                (Some(self_digit), Some(rhs_digit)) => (*self_digit, *rhs_digit),
            };

            let (digit, new_carry) = self_digit + rhs_digit;
            let (digit, carry_carry) = digit + previous_carry;
            let (next_carry, carry_that_is_always_zero) = new_carry + carry_carry;
            assert_eq!(Zero, carry_that_is_always_zero);

            digits.push(digit);
            previous_carry = next_carry;

            i += 1;
        }

        if previous_carry != Zero {
            digits.push(previous_carry);
        }

        Snafu { digits }
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Snafu::default(), |acc, x| acc + x)
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let sum = input.lines().map(parse_snafu_number).sum();
    let to_and_from_decimal = as_snafu(sum);

    let direct: Snafu = input.lines().map(Snafu::from).sum();
    assert_eq!(to_and_from_decimal, direct.to_string());

    println!("part1 = {direct}");
}
