use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve<'input> {
    name: &'input str,
    flow_rate: usize,
    neighbors: Vec<&'input str>,
}

impl<'input> From<&'input str> for Valve<'input> {
    fn from(input: &'input str) -> Self {
        let mut words = input.split(' ');

        let name = words.nth(1).unwrap();
        let flow_rate = words.nth(2).unwrap();

        let mut neighbors: Vec<_> = input.rsplit(", ").collect();
        let last_neighbor = neighbors.last().unwrap();
        let last_neighbor = &last_neighbor[last_neighbor.len() - 2..];
        *neighbors.last_mut().unwrap() = last_neighbor;

        let flow_rate = flow_rate.strip_prefix("rate=").unwrap();
        let flow_rate = flow_rate.strip_suffix(';').unwrap();
        let flow_rate = flow_rate.parse().unwrap();

        Self {
            name,
            flow_rate,
            neighbors,
        }
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let valves: HashMap<&str, Valve> = input
        .lines()
        .map(|line| {
            let valve = Valve::from(line);
            let name = valve.name;
            (name, valve)
        })
        .collect();

    let distances = floyd_warshall(&valves);
    lets_go(&valves, &distances);
}

// minutes remaining (usize), current valve (&str), unopened valves (usize)
type State<'input> = (usize, &'input str, usize);

// fn lets_go(valves: &[Valve], distances: &Distances) {
fn lets_go(valves: &HashMap<&str, Valve>, distances: &Distances) {
    // State:
    // - Current valve (string)
    // - Minutes remaining (int)
    // - Unopened valves (set)
    //
    let mut states: HashMap<State, usize> = HashMap::default();

    let mut useful_valves: Vec<&str> = valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .map(|valve| valve.name)
        .collect();
    useful_valves.sort();
    let useful_valves = useful_valves;

    let _map_of_valves_to_bits: HashMap<&str, usize> = useful_valves
        .iter()
        .enumerate()
        .map(|(i, name)| (*name, i))
        .collect();

    let minutes_remaining = 0;
    for current_valve in valves.values() {
        for i in 0..2usize.pow(useful_valves.len() as u32) {
            let key = (minutes_remaining, current_valve.name, i);
            states.insert(key, 0);
        }
    }

    // for minutes_remaining in 1usize..=30 {
    for minutes_remaining in 1usize..=30 {
        dbg!(minutes_remaining);
        // for minutes_remaining in 1usize..=10 {
        for current_valve in valves.values() {
            // for current_valve in valves.values().filter(|valve| valve.name == "BB") {
            for i in 0..2usize.pow(useful_valves.len() as u32) {
                // for i in 63..=63 {
                // println!("{minutes_remaining}, {}, {i}", current_valve.name);
                // let key = (minutes_remaining - 1, current_valve.name, i);
                // let current_state = states.get(&key).unwrap();

                // Calculate the nil move (do nothing, reap)
                // Iterate over possible moves
                // i.e., all unopened valves
                // - calculate how much moving to that valve would generate
                //   - (minutes remaining - 1 - distance) * flow rate
                // - lookup best possible outcome after that move
                //   - minutes remaining = minutes remaining - 1 - distance
                //   - unopened = unopened - that valve
                // - add would_generate and best_possible_outcome
                // - select max of that sum over all possible moves

                let valveset = usize_to_valveset(i);
                let unopened_valves = valveset.unopened;
                let opened_valves = valveset.opened;

                let previous_key = (minutes_remaining - 1, current_valve.name, i);
                let previous = states.get(&previous_key).unwrap();
                let do_nothing = currently_producing(valves, &opened_valves) + previous;

                let mut best_move = do_nothing;
                for unopened_valve in unopened_valves.iter() {
                    let distance_to = distances
                        .get(&(current_valve.name, unopened_valve))
                        .unwrap();

                    let minutes_remaining_after_move = match minutes_remaining
                        .checked_sub(1)
                        .and_then(|x| x.checked_sub(*distance_to))
                    {
                        Some(x) => x,
                        None => continue,
                    };

                    let to_remove: HashSet<&str> = [*unopened_valve].into();
                    let new_unopened: HashSet<&str> =
                        unopened_valves.difference(&to_remove).copied().collect();
                    let i_after_move = unopened_to_usize(&new_unopened);

                    let next_state = (minutes_remaining_after_move, *unopened_valve, i_after_move);
                    let best_outcome_after_move = states.get(&next_state).unwrap();

                    let produced_along_the_way = (minutes_remaining - minutes_remaining_after_move)
                        * currently_producing(valves, &opened_valves);
                    let final_stuff = best_outcome_after_move + produced_along_the_way;

                    if final_stuff > best_move {
                        best_move = final_stuff;
                    }
                }

                let key = (minutes_remaining, current_valve.name, i);
                states.insert(key, best_move);
            }
        }
    }

    // println!("{}", states.get(&(30, "AA", 0x3F)).unwrap());
    println!("{}", states.get(&(30, "AA", 0x7FFF)).unwrap());

    // for ((minutes_remaining, current_valve, unopened_valves), produced) in states {
    //     println!("({minutes_remaining}, {current_valve}, {unopened_valves}): {produced}");
    // }
}

type Distances<'input> = HashMap<(&'input str, &'input str), usize>;

fn floyd_warshall<'input>(valves: &'input HashMap<&str, Valve>) -> Distances<'input> {
    let mut distances: Distances = HashMap::default();

    for valve in valves.values() {
        for neighbor in &valve.neighbors {
            distances.insert((valve.name, neighbor), 1);
            distances.insert((neighbor, valve.name), 1);
        }

        distances.insert((valve.name, valve.name), 0);
    }

    for k in valves.values() {
        for i in valves.values() {
            for j in valves.values() {
                let to_k = distances.get(&(i.name, k.name)).unwrap_or(&usize::MAX);
                let from_k = distances.get(&(k.name, j.name)).unwrap_or(&usize::MAX);
                let direct = distances.get(&(i.name, j.name)).unwrap_or(&usize::MAX);
                let through = to_k.checked_add(*from_k).unwrap_or(usize::MAX);

                if *direct > through {
                    distances.insert((i.name, j.name), through);
                    distances.insert((j.name, i.name), through);
                }
            }
        }
    }

    distances
}

#[derive(Debug, Default)]
struct ValveSet<'input> {
    opened: HashSet<&'input str>,
    unopened: HashSet<&'input str>,
}

fn usize_to_valveset<'input>(number: usize) -> ValveSet<'input> {
    // assert!(number <= 0x3F);
    // const USEFUL_VALVES: [&str; 6] = ["BB", "CC", "DD", "EE", "HH", "JJ"];
    assert!(number <= 0x7FFF);
    const USEFUL_VALVES: [&str; 15] = [
        "QN", "DS", "AE", "SE", "IJ", "GJ", "EU", "UK", "QB", "MN", "CO", "CS", "NA", "KF", "XM",
    ];
    let mut set = ValveSet::default();

    for (i, valve) in USEFUL_VALVES.into_iter().enumerate() {
        let bit = (number >> i) & 1;
        if bit == 0 {
            set.opened.insert(valve);
        } else {
            set.unopened.insert(valve);
        }
    }

    set
}

fn unopened_to_usize(unopened: &HashSet<&str>) -> usize {
    // const USEFUL_VALVES: [&str; 6] = ["BB", "CC", "DD", "EE", "HH", "JJ"];
    const USEFUL_VALVES: [&str; 15] = [
        "QN", "DS", "AE", "SE", "IJ", "GJ", "EU", "UK", "QB", "MN", "CO", "CS", "NA", "KF", "XM",
    ];

    let mut number = 0;

    for (i, valve) in USEFUL_VALVES.into_iter().enumerate() {
        if unopened.contains(valve) {
            number |= 1 << i;
        }
    }

    number
}

fn currently_producing(valves: &HashMap<&str, Valve>, opened_valves: &HashSet<&str>) -> usize {
    opened_valves
        .iter()
        .map(|name| valves.get(name).unwrap().flow_rate)
        .sum()
}
