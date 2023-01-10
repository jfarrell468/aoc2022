use rangemap::RangeInclusiveSet;
use std::collections::HashSet;
use std::ops::RangeInclusive;
struct Sensor {
    loc: (i32, i32),
    closest_beacon: (i32, i32),
}
impl Sensor {
    fn distance_to(&self, loc: (i32, i32)) -> i32 {
        manhattan_distance(loc, self.loc)
    }
    fn distance_to_beacon(&self) -> i32 {
        self.distance_to(self.closest_beacon)
    }
    fn impossible_range(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let contract = (self.loc.1 - y).abs();
        if contract > self.distance_to_beacon() {
            None
        } else {
            Some(RangeInclusive::new(
                self.loc.0 - (self.distance_to_beacon() - contract),
                self.loc.0 + (self.distance_to_beacon() - contract),
            ))
        }
    }
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

struct SensorNetwork {
    ss: Vec<Sensor>,
    sensor_locs: HashSet<(i32, i32)>,
    beacon_locs: HashSet<(i32, i32)>,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}
impl SensorNetwork {
    fn new() -> SensorNetwork {
        SensorNetwork {
            ss: Vec::new(),
            sensor_locs: HashSet::new(),
            beacon_locs: HashSet::new(),
            xmin: 0,
            xmax: 0,
            ymin: 0,
            ymax: 0,
        }
    }
    fn add_sensor(&mut self, s: Sensor) {
        if self.ss.is_empty() {
            self.xmin = std::cmp::min(s.loc.0, s.closest_beacon.0);
            self.xmax = std::cmp::max(s.loc.0, s.closest_beacon.0);
            self.ymin = std::cmp::min(s.loc.1, s.closest_beacon.1);
            self.ymax = std::cmp::max(s.loc.1, s.closest_beacon.1);
        } else {
            self.xmin = std::cmp::min(self.xmin, s.loc.0 - s.distance_to_beacon());
            self.xmin = std::cmp::min(self.xmin, s.closest_beacon.0);
            self.xmax = std::cmp::max(self.xmax, s.loc.0 + s.distance_to_beacon());
            self.xmax = std::cmp::max(self.xmax, s.closest_beacon.0);
            self.ymin = std::cmp::min(self.ymin, s.loc.1 - s.distance_to_beacon());
            self.ymin = std::cmp::min(self.ymin, s.closest_beacon.1);
            self.ymax = std::cmp::max(self.ymax, s.loc.1 + s.distance_to_beacon());
            self.ymax = std::cmp::max(self.ymax, s.closest_beacon.1);
        }
        self.sensor_locs.insert(s.loc);
        self.beacon_locs.insert(s.closest_beacon);
        self.ss.push(s);
    }
    fn has_sensor_at(&self, pos: (i32, i32)) -> bool {
        self.sensor_locs.contains(&pos)
    }
    fn has_beacon_at(&self, pos: (i32, i32)) -> bool {
        self.beacon_locs.contains(&pos)
    }
    fn print(&self) {
        println!(
            "xmin = {}, xmax = {}, diff = {}",
            self.xmin,
            self.xmax,
            self.xmax - self.xmin + 1
        );
        println!(
            "ymin = {}, ymax = {}, diff = {}",
            self.ymin,
            self.ymax,
            self.ymax - self.ymin + 1
        );
        // for y in self.ymin..=self.ymax {
        //     print!("{:>10} ", y);
        //     for x in self.xmin..=self.xmax {
        //         if self.has_sensor_at((x,y)) {
        //             print!("S");
        //         } else if self.has_beacon_at((x,y)) {
        //             print!("B");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
    }
}

fn main() {
    let mut sensors = SensorNetwork::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let sx = tokens[2]
            .strip_suffix(",")
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let sy = tokens[3]
            .strip_suffix(":")
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let bx = tokens[8]
            .strip_suffix(",")
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();
        let by = tokens[9].split_once("=").unwrap().1.parse::<i32>().unwrap();
        let s = Sensor {
            loc: (sx, sy),
            closest_beacon: (bx, by),
        };
        println!(
            "sensor: {:?}, closest beacon: {:?}, distance: {}",
            s.loc,
            s.closest_beacon,
            s.distance_to_beacon()
        );
        sensors.add_sensor(s);
    }
    sensors.print();
    let y = 2000000;
    let mut num_invalid = 0;
    // let mut row = String::from("           ");
    for x in sensors.xmin..=sensors.xmax {
        // println!("Checking {:?}", (x, y));
        let mut valid = true;
        for sensor in &sensors.ss {
            // println!("  Sensor position: {:?}, closest beacon: {:?}, distance to beacon: {}, distance to {:?}: {}", sensor.loc, sensor.closest_beacon, sensor.distance_to_beacon(), (x,y), sensor.distance_to((x,y)));
            if sensor.distance_to((x, y)) <= sensor.distance_to_beacon()
                && (x, y) != sensor.loc
                && (x, y) != sensor.closest_beacon
            {
                // println!("    Not valid");
                valid = false;
                break;
            }
            // println!("    Valid");
        }
        if !valid {
            num_invalid += 1;
        }
        // row.push(if valid { '.' } else { '#' });
    }
    // println!("{}", row);
    println!("Part 1: {}", num_invalid);

    for y in 0..=4000000 {
        // println!("y = {}", y);
        let mut impossible = RangeInclusiveSet::new();
        for sensor in &sensors.ss {
            if let Some(r) = sensor.impossible_range(y) {
                impossible.insert(r);
            }
        }
        if impossible.len() > 1 {
            println!("{:?}, {}", impossible, impossible.len());
            println!(
                "{}",
                (y as i64) + 4000000 * (*impossible.iter().next().unwrap().end() as i64 + 1)
            );
            break;
        }
    }
}
