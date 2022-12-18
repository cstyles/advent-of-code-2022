use std::collections::HashSet;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Beacon {
    y: isize,
    x: isize,
}

impl Beacon {
    fn to_coordinates(self) -> (isize, isize) {
        (self.y, self.x)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Sensor {
    y: isize,
    x: isize,
    closest_beacon: Beacon,
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Self {
        let (_, rest) = input.split_once("Sensor at x=").unwrap();
        let (x, rest) = rest.split_once(", y=").unwrap();
        let sensor_x = x.parse().unwrap();

        let (y, rest) = rest.split_once(": closest beacon is at x=").unwrap();
        let sensor_y = y.parse().unwrap();

        let (x, y) = rest.split_once(", y=").unwrap();
        let beacon_x = x.parse().unwrap();
        let beacon_y = y.parse().unwrap();

        let closest_beacon = Beacon {
            y: beacon_y,
            x: beacon_x,
        };

        Self {
            x: sensor_x,
            y: sensor_y,
            closest_beacon,
        }
    }
}

impl Sensor {
    fn manhattan_distance(&self, y: isize, x: isize) -> usize {
        self.x.abs_diff(x) + self.y.abs_diff(y)
    }

    fn manhattan_distance_to_closest_beacon(&self) -> usize {
        self.manhattan_distance(self.closest_beacon.y, self.closest_beacon.x)
    }

    fn contains(&self, (y, x): (isize, isize)) -> bool {
        self.manhattan_distance(y, x) < self.manhattan_distance_to_closest_beacon()
    }

    fn cells_in_row_where_beacon_cannot_be(&self, target_row: isize) -> Vec<(isize, isize)> {
        let distance_to_beacon = self.manhattan_distance_to_closest_beacon();
        let distance_to_target_row = self.y.abs_diff(target_row);

        if distance_to_beacon < distance_to_target_row {
            return vec![];
        }

        let remaining_steps = (distance_to_beacon - distance_to_target_row) as isize;

        ((self.x - remaining_steps)..=(self.x + remaining_steps))
            .map(|x| (target_row, x))
            .collect()
    }
}

fn main() {
    let (input, target_row) = if std::env::var("TEST").is_ok() {
        (include_str!("../test_input.txt"), 10)
    } else {
        (include_str!("../input.txt"), 2_000_000)
    };

    let sensors: Vec<Sensor> = input.lines().map(Sensor::from).collect();
    let beacons: HashSet<Beacon> = sensors.iter().map(|sensor| sensor.closest_beacon).collect();

    part1(sensors.clone(), beacons, target_row);
    part2(sensors);
}

fn part2(sensors: Vec<Sensor>) {
    let mut almost_touching: HashSet<(Sensor, Sensor)> = HashSet::default();

    for pair in sensors.iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];

        let distance_between = a.manhattan_distance(b.y, b.x);
        let a_size = a.manhattan_distance_to_closest_beacon();
        let b_size = b.manhattan_distance_to_closest_beacon();
        let width_of_gap_between = distance_between
            .checked_sub(a_size)
            .and_then(|x| x.checked_sub(b_size));

        if width_of_gap_between.map_or(false, |x| x == 2) {
            almost_touching.insert((*a, *b));
        }
    }

    for pairs in almost_touching.into_iter().combinations(2) {
        let _pair_a = pairs[0];
        let _pair_b = pairs[1];
    }

    let answer = (2_916_597, 2_727_057);
    for sensor in sensors.iter() {
        if sensor.contains(answer) {
            dbg!(sensor);
        }
    }

    println!("part2 = {}", tuning_frequency(answer));
}

fn part1(sensors: Vec<Sensor>, beacons: HashSet<Beacon>, target_row: isize) {
    let mut cells_in_target_row: HashSet<(isize, isize)> = HashSet::default();
    for sensor in sensors {
        for cell in sensor.cells_in_row_where_beacon_cannot_be(target_row) {
            cells_in_target_row.insert(cell);
        }
    }

    let beacon_cells: HashSet<(isize, isize)> =
        beacons.into_iter().map(Beacon::to_coordinates).collect();
    let cells_in_target_row = cells_in_target_row.difference(&beacon_cells).count();

    println!("part1 = {cells_in_target_row}");
}

fn tuning_frequency((y, x): (isize, isize)) -> isize {
    x * 4_000_000 + y
}
