#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let (_move, rest) = input.split_once(' ').unwrap();
        let (count, rest) = rest.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();

        let count = count.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        Self {
            count,
            from: from - 1,
            to: to - 1,
        }
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let (towers, moves) = input.split_once("\n\n").unwrap();

    let rows: Vec<Vec<char>> = towers
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect();

    let number_of_towers = rows.last().unwrap().len();
    let mut towers = vec![vec![]; number_of_towers];

    // Skip last row since it's the labels (1, 2, 3, etc.)
    for row in rows.into_iter().rev().skip(1) {
        for (i, character) in row.into_iter().enumerate().filter(|(_i, c)| *c != ' ') {
            towers[i].push(character);
        }
    }

    let moves: Vec<Move> = moves.lines().map(Move::from).collect();

    play(&mut towers, &moves);

    let part1: String = towers.iter().map(|tower| tower.last().unwrap()).collect();
    println!("part1 = {part1}");
}

fn play(towers: &mut [Vec<char>], moves: &[Move]) {
    for move_ in moves {
        for _ in 0..move_.count {
            let from = &mut towers[move_.from];
            let top = from.pop().unwrap();
            let to = &mut towers[move_.to];
            to.push(top);
        }
    }
}
