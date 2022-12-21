fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let numbers: Vec<(usize, i64)> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .enumerate()
        .collect();

    let part1 = decrypt(numbers.clone(), 1);
    println!("part1 = {part1}");

    let numbers: Vec<_> = numbers
        .into_iter()
        .map(|(i, num)| (i, num * 811_589_153))
        .collect();
    let part2 = decrypt(numbers, 10);
    println!("part2 = {part2}");
}

fn decrypt(mut numbers: Vec<(usize, i64)>, cycles: usize) -> i64 {
    for _ in 0..cycles {
        mix(&mut numbers);
    }

    numbers
        .iter()
        .map(|(_, number)| number)
        .copied()
        .cycle()
        .skip_while(|number| *number != 0)
        .step_by(1_000)
        .skip(1)
        .take(3)
        .sum()
}

fn mix(numbers: &mut Vec<(usize, i64)>) {
    let length = numbers.len();
    for index in 0..length {
        let index_of_number_to_move = numbers.iter().position(|(i, _)| *i == index).unwrap();
        let original_number_to_move = numbers[index_of_number_to_move].1;
        let mut how_much_to_move = original_number_to_move % (length - 1) as i64;

        if how_much_to_move.is_positive() {
            let wrap_off_right = index_of_number_to_move as i64 + how_much_to_move >= length as i64;
            if wrap_off_right {
                // turn into negative
                how_much_to_move = how_much_to_move - length as i64 + 1;
            }
        } else if how_much_to_move.is_negative() {
            let wrap_off_left = index_of_number_to_move as i64 + how_much_to_move <= 0;
            if wrap_off_left {
                // turn into positive
                how_much_to_move = length as i64 + how_much_to_move - 1;
            }
        } else {
            continue;
        }

        if how_much_to_move.is_positive() {
            let target = index_of_number_to_move + how_much_to_move as usize;
            let start = index_of_number_to_move + 1;
            let range = start..=target;
            numbers.copy_within(range, index_of_number_to_move);
            numbers[target] = (index, original_number_to_move);
        } else {
            let target = (index_of_number_to_move as i64 + how_much_to_move) as usize;
            let range = target..index_of_number_to_move;
            numbers.copy_within(range, target + 1);
            numbers[target] = (index, original_number_to_move);
        }
    }
}
