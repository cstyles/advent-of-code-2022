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

    for blueprint in &blueprints[..1] {
        dbg!(test_blueprint(*blueprint));
    }
    // dbg!(blueprints);
}

fn test_blueprint(blueprint: Blueprint) -> usize {
    let mut ore_robots = 1;
    let mut clay_robots = 0;
    let mut obsidian_robots = 0;
    let mut geode_robots = 0;

    let mut ore = 0;
    let mut clay = 0;
    let mut obsidian = 0;
    let mut geodes = 0;

    let mut minute = 1;
    while minute <= 24 {
        let mut new_geode_robots = 0;
        let mut new_obsidian_robots = 0;
        let mut new_clay_robots = 0;
        let mut new_ore_robots = 0;

        println!("== Minute {minute} ==");

        if ore >= blueprint.geode_robot_ore_cost && obsidian >= blueprint.geode_robot_obsidian_cost
        {
            println!("Build geode robot");
            new_geode_robots += 1;
            ore -= blueprint.geode_robot_ore_cost;
            obsidian -= blueprint.geode_robot_obsidian_cost;
        }

        if ore >= blueprint.obsidian_robot_ore_cost && clay >= blueprint.obsidian_robot_clay_cost {
            println!("Build obsidian robot");
            new_obsidian_robots += 1;
            ore -= blueprint.obsidian_robot_ore_cost;
            clay -= blueprint.obsidian_robot_clay_cost;
        }

        if ore >= blueprint.clay_robot_cost {
            println!(
                "Spend {} ore to start building a clay-collecting robot.",
                blueprint.clay_robot_cost
            );
            new_clay_robots += 1;
            ore -= blueprint.clay_robot_cost;
        }

        if ore >= blueprint.ore_robot_cost {
            println!(
                "Spend {} ore to start building a clay-collecting robot.",
                blueprint.ore_robot_cost
            );
            new_ore_robots += 1;
            ore -= blueprint.ore_robot_cost;
        }

        ore += ore_robots;
        println!(
            "{ore_robots} ore-collecting robot collects {ore_robots} ore; you now have {ore} ore."
        );
        clay += clay_robots;
        println!("{clay_robots} clay-collecting robot collects {clay_robots} ore; you now have {clay} clay.");
        obsidian += obsidian_robots;
        println!("{obsidian_robots} obsidian-collecting robot collects {obsidian_robots} ore; you now have {obsidian} obsidian.");
        geodes += geode_robots;
        println!("{geode_robots} geode-collecting robot collects {geode_robots} ore; you now have {geodes} open geodes.");

        geode_robots += new_geode_robots;
        obsidian_robots += new_obsidian_robots;
        clay_robots += new_clay_robots;
        ore_robots += new_ore_robots;

        minute += 1;
        println!();
    }

    geodes
}
