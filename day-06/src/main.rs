use std::collections::HashSet;

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let chars: Vec<_> = input.chars().collect();

    solve(1, &chars, 4);
    solve(2, &chars, 14);
}

fn solve(part: usize, chars: &[char], unique: usize) {
    for (i, window) in chars.windows(unique).enumerate() {
        let set: HashSet<_> = window.iter().collect();
        if set.len() == unique {
            println!("part{} = {}", part, i + unique);
            break;
        }
    }
}
