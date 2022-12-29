use std::collections::{HashMap, VecDeque};

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
    let start = valves
        .iter()
        .position(|valve| valve.debug_name == "AA")
        .unwrap();

    let map_of_debug_name_to_index: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(index, valve)| (valve.debug_name, index))
        .collect();

    let real_valves: Vec<RealValve> = valves
        .into_iter()
        .map(|old_valve| {
            let neighbors = old_valve
                .neighbors
                .into_iter()
                .map(|neighbor| *map_of_debug_name_to_index.get(neighbor).unwrap())
                .collect();

            RealValve {
                flow_rate: old_valve.flow_rate,
                neighbors,
            }
        })
        .collect();

    let distances = floyd_warshall(&real_valves);
    part1(&real_valves, &distances, start);
}

struct State {
    minutes_remaining: usize,
    valveset: BitSet,
    current_valve: usize,
    relieved_so_far: usize,
}

fn part1(valves: &[RealValve], distances: &Distances, start: usize) {
    let mut max_relieved_states: HashMap<BitSet, usize> = HashMap::default();
    let mut queue: VecDeque<State> = VecDeque::new();

    let size = valves.len();
    let num = usize::MAX ^ usize::MAX << size;
    let valveset = BitSet { num, size };

    queue.push_back(State {
        minutes_remaining: 26,
        valveset,
        current_valve: start,
        relieved_so_far: 0,
    });

    while let Some(State {
        minutes_remaining,
        valveset,
        current_valve,
        relieved_so_far,
    }) = queue.pop_front()
    {
        // Try waiting until the end
        let do_nothing = wait_until_end(valves, minutes_remaining, valveset);
        max_relieved_states
            .entry(valveset)
            .and_modify(|max_relieved| {
                *max_relieved = (*max_relieved).max(relieved_so_far + do_nothing)
            })
            .or_insert(relieved_so_far + do_nothing);

        for valve in valveset
            .unopened()
            .into_iter()
            .filter(|idx| valves[*idx].flow_rate > 0)
        {
            let new_valveset = valveset.without(valve);
            let how_long_to_valve = distances[current_valve][valve] + 1;

            if how_long_to_valve >= minutes_remaining {
                continue;
            }

            let relieved_along_the_way = total_flowrate(valves, valveset) * how_long_to_valve;
            let relieved_so_far = relieved_so_far + relieved_along_the_way;

            queue.push_back(State {
                minutes_remaining: minutes_remaining - how_long_to_valve,
                valveset: new_valveset,
                current_valve: valve,
                relieved_so_far,
            });
        }
    }

    let part1 = max_relieved_states.values().max().unwrap();
    println!("part1 = {part1}");

    let relevant: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_i, valve)| valve.flow_rate > 0)
        .map(|(i, _valve)| i)
        .collect();

    let mut part2 = 0;
    for (i, (elephant_set, elephant_max)) in max_relieved_states.iter().enumerate() {
        for (human_set, human_max) in max_relieved_states.iter().skip(i) {
            let elephant_opened = elephant_set.relevant_opened(&relevant);
            let human_opened = human_set.relevant_opened(&relevant);

            if !disjoint(&elephant_opened, &human_opened) {
                continue;
            }

            part2 = part2.max(elephant_max + human_max);
        }
    }

    println!("part2 = {part2}");
}

fn total_flowrate(valves: &[RealValve], valveset: BitSet) -> usize {
    valveset
        .opened()
        .into_iter()
        .map(|idx| valves[idx].flow_rate)
        .sum()
}

fn wait_until_end(valves: &[RealValve], minutes_remaining: usize, valveset: BitSet) -> usize {
    minutes_remaining * total_flowrate(valves, valveset)
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

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct BitSet {
    num: usize,
    size: usize,
}

impl BitSet {
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

    fn relevant_opened(&self, relevant: &[usize]) -> Vec<usize> {
        self.opened()
            .into_iter()
            .filter(|bit| relevant.contains(bit))
            .collect()
    }

    fn without(self, bit: usize) -> Self {
        let mask = !(1 << bit);
        let num = self.num & mask;

        Self { num, ..self }
    }
}

fn disjoint<T: PartialEq>(v1: &[T], v2: &[T]) -> bool {
    for x in v1 {
        if v2.contains(x) {
            return false;
        }
    }

    true
}
