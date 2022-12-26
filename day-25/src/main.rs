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

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let sum = input.lines().map(parse_snafu_number).sum();
    let part1 = as_snafu(sum);
    println!("part1 = {part1}");
}
