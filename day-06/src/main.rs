use std::collections::HashSet;

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let chars: Vec<_> = input.chars().collect();

    for (i, window) in chars.windows(4).enumerate() {
        let set: HashSet<_> = window.iter().collect();
        if set.len() == 4 {
            println!("part1 = {}", i + 4);
            break;
        }
    }
}
