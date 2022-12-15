use std::collections::HashSet;

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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    fn manhattan_distance_to_closest_beacon(&self) -> usize {
        self.x.abs_diff(self.closest_beacon.x) + self.y.abs_diff(self.closest_beacon.y)
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
