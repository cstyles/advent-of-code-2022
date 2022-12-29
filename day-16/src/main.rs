use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve<'input> {
    debug_name: &'input str,
    flow_rate: usize,
    neighbors: Vec<&'input str>,
}

impl<'input> From<&'input str> for Valve<'input> {
    fn from(input: &'input str) -> Self {
        let mut words = input.split(' ');

        let debug_name = words.nth(1).unwrap();
        let flow_rate = words.nth(2).unwrap();

        let mut neighbors: Vec<_> = input.rsplit(", ").collect();
        let last_neighbor = neighbors.last().unwrap();
        let last_neighbor = &last_neighbor[last_neighbor.len() - 2..];
        *neighbors.last_mut().unwrap() = last_neighbor;

        let flow_rate = flow_rate.strip_prefix("rate=").unwrap();
        let flow_rate = flow_rate.strip_suffix(';').unwrap();
        let flow_rate = flow_rate.parse().unwrap();

        Self {
            debug_name,
            flow_rate,
            neighbors,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RealValve {
    flow_rate: usize,
    neighbors: Vec<usize>,
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let valves: Vec<Valve> = input.lines().map(Valve::from).collect();

    let map_of_debug_name_to_namecode: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(name, valve)| (valve.debug_name, name))
        .collect();

    let real_valves: Vec<RealValve> = valves
        .into_iter()
        .map(|old_valve| {
            let neighbors = old_valve
                .neighbors
                .into_iter()
                .map(|neighbor| *map_of_debug_name_to_namecode.get(neighbor).unwrap())
                .collect();

            RealValve {
                flow_rate: old_valve.flow_rate,
                neighbors,
            }
        })
        .collect();

    let distances = floyd_warshall(&real_valves);
    lets_go(&real_valves, &distances);
}

// minutes remaining, current valve, unopened valves
type States = Vec<Vec<Vec<usize>>>;

fn lets_go(valves: &[RealValve], distances: &Distances) {
    let size = if std::env::var("TEST").is_ok() { 6 } else { 15 };
    let mut states = vec![vec![vec![0; 2usize.pow(size as u32)]; valves.len()]; 31];

    let useful_valve_names: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_i, valve)| valve.flow_rate > 0)
        .map(|(i, _valve)| i)
        .collect();

    let minutes_remaining = 0;
    for name in 0..valves.len() {
        for i in 0..2usize.pow(useful_valve_names.len() as u32) {
            states[minutes_remaining][name][i] = 0;
        }
    }

    for minutes_remaining in 1usize..=30 {
        dbg!(minutes_remaining);
        for &current_valve_name in useful_valve_names.iter() {
            for i in 0..2usize.pow(useful_valve_names.len() as u32) {
                let best_move = find_best_move(
                    valves,
                    &useful_valve_names,
                    distances,
                    &states,
                    minutes_remaining,
                    current_valve_name,
                    i,
                    size,
                );
                states[minutes_remaining][current_valve_name][i] = best_move;
            }
        }
    }

    let part1 = if std::env::var("TEST").is_ok() {
        find_best_move(
            valves,
            &useful_valve_names,
            distances,
            &states,
            30,
            0,
            0x3F,
            size,
        )
    } else {
        find_best_move(
            valves,
            &useful_valve_names,
            distances,
            &states,
            30,
            14,
            0x7FFF,
            size,
        )
    };

    println!("part1 = {part1}");

    // for ((minutes_remaining, current_valve, unopened_valves), produced) in states {
    //     println!("({minutes_remaining}, {current_valve}, {unopened_valves}): {produced}");
    // }
}

fn find_best_move(
    valves: &[RealValve],
    useful_valve_names: &[usize],
    distances: &Distances,
    states: &States,
    minutes_remaining: usize,
    current_valve_name: usize,
    i: usize,
    size: usize,
) -> usize {
    let valveset = BitSet { num: i, size };
    let unopened_valves = valveset.unopened();
    let opened_valves = valveset.opened();
    let opened_valves: Vec<usize> = opened_valves
        .into_iter()
        .map(|useful_valve| useful_valve_names[useful_valve])
        .collect();

    let mut best_move = None;
    for &unopened_valve in unopened_valves.iter() {
        let real_name = useful_valve_names[unopened_valve];

        let distance_to = distances[current_valve_name][real_name];

        let minutes_remaining_after_move = match minutes_remaining
            .checked_sub(1)
            .and_then(|x| x.checked_sub(distance_to))
        {
            Some(x) => x,
            None => continue,
        };

        let i_after_move = BitSet { num: i, size }.without(unopened_valve).num;
        let best_outcome_after_move = states[minutes_remaining_after_move][real_name][i_after_move];

        let produced_along_the_way = (minutes_remaining - minutes_remaining_after_move)
            * currently_producing(valves, &opened_valves);
        let final_stuff = best_outcome_after_move + produced_along_the_way;

        if best_move
            .map(|best_move| final_stuff > best_move)
            .unwrap_or(true)
        {
            best_move = Some(final_stuff);
        }
    }

    best_move.unwrap_or_else(|| {
        let previous = states[minutes_remaining - 1][current_valve_name][i];
        currently_producing(valves, &opened_valves) + previous
    })
}

type Distances = Vec<Vec<usize>>;

fn floyd_warshall(valves: &[RealValve]) -> Distances {
    let mut distances: Distances = vec![vec![usize::MAX; valves.len()]; valves.len()];

    for (name, valve) in valves.iter().enumerate() {
        for &neighbor in &valve.neighbors {
            distances[name][neighbor] = 1;
            distances[neighbor][name] = 1;
        }

        distances[name][name] = 0;
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let to_k = distances[i][k];
                let from_k = distances[k][j];
                let direct = distances[i][j];
                let through = to_k.checked_add(from_k).unwrap_or(usize::MAX);

                if direct > through {
                    distances[i][j] = through;
                    distances[j][i] = through;
                }
            }
        }
    }

    distances
}

fn currently_producing(valves: &[RealValve], opened: &[usize]) -> usize {
    opened.iter().map(|&num| valves[num].flow_rate).sum()
}

#[derive(Debug, Default, Copy, Clone)]
struct BitSet {
    num: usize,
    size: usize,
}

impl BitSet {
    fn contains(&self, bit: usize) -> bool {
        (self.num >> bit) & 1 != 0
    }

    fn unopened(&self) -> Vec<usize> {
        let mut bits = vec![];

        let mut set = self.num;
        let mut i = 0;
        while i < self.size {
            if set & 1 != 0 {
                bits.push(i);
            }

            set >>= 1;
            i += 1;
        }

        bits
    }

    fn opened(&self) -> Vec<usize> {
        let mut bits = vec![];

        let mut set = self.num;
        let mut i = 0;
        while i < self.size {
            if set & 1 == 0 {
                bits.push(i);
            }

            set >>= 1;
            i += 1;
        }

        bits
    }

    fn without(self, bit: usize) -> Self {
        let mask = !(1 << bit);
        let num = self.num & mask;

        Self { num, ..self }
    }
}
