use std::collections::BinaryHeap;

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    number: u16,
    ore_robot_cost: u16,
    clay_robot_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        let input = input.strip_prefix("Blueprint ").unwrap();
        let (number, rest) = input.split_once(": Each ore robot costs ").unwrap();
        let (ore_robot_cost, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_robot_cost, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_robot_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
        let (obsidian_robot_clay_cost, rest) =
            rest.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_robot_ore_cost, rest) = rest.split_once(" ore and ").unwrap();
        let (geode_robot_obsidian_cost, _rest) = rest.split_once(' ').unwrap();

        let number = number.parse().unwrap();
        let ore_robot_cost = ore_robot_cost.parse().unwrap();
        let clay_robot_cost = clay_robot_cost.parse().unwrap();
        let obsidian_robot_ore_cost = obsidian_robot_ore_cost.parse().unwrap();
        let obsidian_robot_clay_cost = obsidian_robot_clay_cost.parse().unwrap();
        let geode_robot_ore_cost = geode_robot_ore_cost.parse().unwrap();
        let geode_robot_obsidian_cost = geode_robot_obsidian_cost.parse().unwrap();

        Self {
            number,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        }
    }
}

fn main() {
    let input = if std::env::var("TEST").is_ok() {
        include_str!("../test_input.txt")
    } else {
        include_str!("../input.txt")
    };

    let blueprints: Vec<_> = input.lines().map(Blueprint::from).collect();

    let mut handles = vec![];
    for blueprint in blueprints.clone() {
        let handle = std::thread::spawn(move || test_blueprint::<24>(blueprint));
        handles.push((blueprint.number, handle));
    }

    let mut part1 = 0;
    for (blueprint_number, handle) in handles {
        part1 += blueprint_number * handle.join().unwrap();
    }
    println!("part1 = {part1}");

    let mut handles = vec![];
    for blueprint in blueprints.into_iter().take(3) {
        let handle = std::thread::spawn(move || test_blueprint::<32>(blueprint));
        handles.push(handle);
    }

    let mut part2 = 1;
    for handle in handles {
        let result = handle.join().unwrap();
        part2 *= result;
    }

    println!("part2 = {part2}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    minutes_elapsed: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
}

impl Default for State {
    fn default() -> Self {
        Self {
            minutes_elapsed: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

impl State {
    fn next(self) -> Self {
        Self {
            minutes_elapsed: self.minutes_elapsed + 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..self
        }
    }

    fn build_ore_robot(self, blueprint: Blueprint) -> Self {
        Self {
            ore: self.ore + self.ore_robots - blueprint.ore_robot_cost,
            ore_robots: self.ore_robots + 1,
            ..self.next()
        }
    }

    fn build_clay_robot(self, blueprint: Blueprint) -> Self {
        Self {
            ore: self.ore + self.ore_robots - blueprint.clay_robot_cost,
            clay_robots: self.clay_robots + 1,
            ..self.next()
        }
    }

    fn build_obsidian_robot(self, blueprint: Blueprint) -> Self {
        Self {
            ore: self.ore + self.ore_robots - blueprint.obsidian_robot_ore_cost,
            clay: self.clay + self.clay_robots - blueprint.obsidian_robot_clay_cost,
            obsidian_robots: self.obsidian_robots + 1,
            ..self.next()
        }
    }

    fn build_geode_robot(self, blueprint: Blueprint) -> Self {
        Self {
            ore: self.ore + self.ore_robots - blueprint.geode_robot_ore_cost,
            obsidian: self.obsidian + self.obsidian_robots - blueprint.geode_robot_obsidian_cost,
            geode_robots: self.geode_robots + 1,
            ..self.next()
        }
    }

    fn as_timeless(self) -> TimelessState {
        TimelessState {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geodes: self.geodes,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.geodes.partial_cmp(&other.geodes)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.geodes.cmp(&other.geodes)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct TimelessState {
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geodes: u16,
}

fn test_blueprint<const MAX_MINUTES: u16>(blueprint: Blueprint) -> u16 {
    let mut states: fnv::FnvHashMap<TimelessState, u16> = Default::default();
    let mut heap = BinaryHeap::new();
    heap.push(State::default());
    let mut most_geodes = 0;

    while let Some(state) = heap.pop() {
        // If we've already seen this state (i.e., this amount of resources
        // and robots) previously and we got there in the same amount of time
        // or earlier, just skip this state.
        if let Some(other_time_elapsed) = states.get(&state.as_timeless()) {
            if *other_time_elapsed <= state.minutes_elapsed {
                continue;
            }
        }

        most_geodes = most_geodes.max(state.geodes);

        if state.minutes_elapsed == MAX_MINUTES {
            continue;
        }

        // If there's no way to beat the best score, just give up
        if maximum_possible_geodes::<MAX_MINUTES>(state) <= most_geodes {
            continue;
        }

        states.insert(state.as_timeless(), state.minutes_elapsed);

        // Possible decisions:
        if state.ore >= blueprint.geode_robot_ore_cost
            && state.obsidian >= blueprint.geode_robot_obsidian_cost
        {
            heap.push(state.build_geode_robot(blueprint));
        }

        if !dont_need_to_build_obsidian_robot(state, blueprint)
            && state.ore >= blueprint.obsidian_robot_ore_cost
            && state.clay >= blueprint.obsidian_robot_clay_cost
        {
            heap.push(state.build_obsidian_robot(blueprint));
        }

        if !dont_need_to_build_clay_robot(state, blueprint)
            && state.ore >= blueprint.clay_robot_cost
        {
            heap.push(state.build_clay_robot(blueprint));
        }

        if !dont_need_to_build_ore_robot(state, blueprint) && state.ore >= blueprint.ore_robot_cost
        {
            heap.push(state.build_ore_robot(blueprint));
        }

        heap.push(state.next());
    }

    most_geodes
}

// Returns true if we never need to build another ore robot again
// (e.g., if we're already producing enough ore)
fn dont_need_to_build_ore_robot(state: State, blueprint: Blueprint) -> bool {
    let max_ore_cost = blueprint
        .ore_robot_cost
        .max(blueprint.clay_robot_cost)
        .max(blueprint.obsidian_robot_ore_cost)
        .max(blueprint.geode_robot_ore_cost);

    state.ore_robots >= max_ore_cost
}

fn dont_need_to_build_clay_robot(state: State, blueprint: Blueprint) -> bool {
    state.clay_robots >= blueprint.obsidian_robot_clay_cost
}

fn dont_need_to_build_obsidian_robot(state: State, blueprint: Blueprint) -> bool {
    state.obsidian_robots >= blueprint.geode_robot_obsidian_cost
}

fn maximum_possible_geodes<const MAX_MINUTES: u16>(mut state: State) -> u16 {
    let mut max = state.geodes;
    for _ in state.minutes_elapsed..MAX_MINUTES {
        max += state.geode_robots;
        state.geode_robots += 1;
    }

    max
}
