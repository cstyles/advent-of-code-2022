use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    number: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
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

    // let part1: usize = blueprints.into_iter().map(test_blueprint::<24>).sum();
    // println!("part1 = {part1}");

    let mut handles = vec![];
    for blueprint in blueprints.into_iter().take(3) {
        let handle = std::thread::spawn(move || test_blueprint::<32>(blueprint));
        handles.push(handle);
        // println!(
        //     "most geodes for blueprint #{} = {}",
        //     blueprint.number,
        //     test_blueprint::<32>(*blueprint)
        // );
    }

    let mut part2 = 1;
    for handle in handles {
        let result = handle.join().unwrap();
        part2 *= dbg!(result);
    }

    println!("part2 = {part2}");
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    minutes_elapsed: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
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
}

fn test_blueprint<const MAX_MINUTES: usize>(blueprint: Blueprint) -> usize {
    let mut states: HashSet<State> = [].into(); //State::default()].into();
    let mut queue = VecDeque::new();
    queue.push_back(State::default());
    let mut most_geodes = 0;
    // let mut states_examined_by_minute: HashMap<usize, u64> = Default::default();

    while let Some(state) = queue.pop_front() {
        // states_examined_by_minute
        //     .entry(state.minutes_elapsed)
        //     .and_modify(|e| *e += 1)
        //     .or_insert(1);
        // std::thread::sleep(std::time::Duration::from_millis(500));
        // dbg!(state);

        // If we've already seen this state previously and we've already done
        // as well or better, just skip it.
        if let Some(previous_state) = states.get(&state) {
            if previous_state.geodes >= state.geodes {
                // This skips roughly 76% of states
                // println!("skipping");
                continue;
            }
        }

        most_geodes = most_geodes.max(state.geodes);

        if state.minutes_elapsed == MAX_MINUTES {
            continue;
        }

        // If there's no way to beat the best score, just give up
        if state.geodes + (MAX_MINUTES - state.minutes_elapsed) < most_geodes {
            continue;
        }

        states.insert(state);

        // if states.len() % 1_000_000 == 0 {
        //     println!("{}", states.len());
        // }

        // Possible decisions:
        if state.ore >= blueprint.geode_robot_ore_cost
            && state.obsidian >= blueprint.geode_robot_obsidian_cost
        {
            queue.push_back(state.build_geode_robot(blueprint));
        }

        if state.minutes_elapsed < 28 {
            if state.ore >= blueprint.obsidian_robot_ore_cost
                && state.clay >= blueprint.obsidian_robot_clay_cost
            {
                queue.push_back(state.build_obsidian_robot(blueprint));
            }
        }

        if state.minutes_elapsed < 22 {
            if state.ore >= blueprint.clay_robot_cost {
                queue.push_back(state.build_clay_robot(blueprint));
            }
        }

        if state.minutes_elapsed < 16 {
            if state.ore >= blueprint.ore_robot_cost {
                queue.push_back(state.build_ore_robot(blueprint));
            }
        }

        queue.push_back(state.next());
    }

    // dbg!(states_examined_by_minute);

    most_geodes
}
