use std::fmt;

#[derive(Debug, Clone)]
struct Blueprint {
    id: i32,
    ore_cost: i32,
    clay_cost: i32,
    obs_ore_cost: i32,
    obs_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obs_cost: i32,
}
impl Blueprint {
    fn max_robots(&self, t: usize) -> i32 {
        match t {
            OBSIDIAN => self.geode_obs_cost,
            CLAY => self.obs_clay_cost,
            ORE => self.geode_ore_cost + self.obs_ore_cost + self.clay_cost + self.ore_cost,
            _ => panic!(),
        }
    }
    fn find_best(&self, max_t: i32) -> i32 {
        println!("{:?}", self);

        let bc = BestCase::from(&self, max_t);
        // bc.print();
        if bc.data.last().unwrap().resources[GEODE] == 0 {
            println!("No geodes possible");
            return 0;
        }
        // break;

        let mut start = State::new(max_t);
        start.geode_lower_limit = start.predict_resource(GEODE);
        start.geode_upper_limit = start.geode_upper_bound(&bc);
        let mut ss = Vec::from([start]);
        let mut most_geodes = 0;
        let mut iter: i64 = 0;
        State::print_header();
        while !ss.is_empty() {
            let s = ss.pop().unwrap();
            // println!("{}", s);
            let gll = s.geode_lower_limit;
            let gul = s.geode_upper_limit;
            if gll > most_geodes {
                most_geodes = gll;
                println!("{}", s);
            }
            if gul >= most_geodes && gul > 0 {
                for s in s.next_states(&self, &bc) {
                    // println!("next state");
                    // if s.geode_upper_limit <= 0 { println!("{}", s); }
                    if s.geode_upper_limit >= most_geodes && s.geode_upper_limit > 0 {
                        ss.push(s);
                    }
                }
            }
            iter += 1;
            if iter % 100000000 == 0 {
                println!("{} iterations, {} items in stack", iter, ss.len());
            }
            // if iter > 100 {
            //     break;
            // }
        }
        most_geodes
    }
}

const GEODE: usize = 0;
const OBSIDIAN: usize = 1;
const CLAY: usize = 2;
const ORE: usize = 3;
const ALL_TYPES: [usize; 4] = [ORE, CLAY, OBSIDIAN, GEODE];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    geode_upper_limit: i32,
    geode_lower_limit: i32,
    robots: [i32; 4],
    resources: [i32; 4],
    time: i32,
    tr: i32,
}
impl State {
    fn new(max_t: i32) -> State {
        State {
            geode_lower_limit: 0,
            geode_upper_limit: 0,
            robots: [0, 0, 0, 1],
            resources: [0, 0, 0, 0],
            time: 0,
            tr: max_t,
        }
    }
    fn print_header() {
        println!("+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+");
        println!(
            "|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|{:>9}|",
            "time",
            "tr",
            "g ll",
            "g ul",
            "geodes",
            "geode r",
            "obs",
            "obs r",
            "clay",
            "clay r",
            "ore",
            "ore r"
        );
        println!("+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+---------+");
    }
    fn can_build_n(&self, t: usize, b: &Blueprint, n: i32) -> bool {
        if t != GEODE && self.robots[t] >= b.max_robots(t) {
            false
        } else {
            match t {
                GEODE => {
                    self.resources[ORE] >= n * b.geode_ore_cost
                        && self.resources[OBSIDIAN] >= n * b.geode_obs_cost
                }
                OBSIDIAN => {
                    self.resources[ORE] >= n * b.obs_ore_cost
                        && self.resources[CLAY] >= n * b.obs_clay_cost
                }
                CLAY => self.resources[ORE] >= n * b.clay_cost,
                ORE => self.resources[ORE] >= n * b.ore_cost,
                _ => panic!(),
            }
        }
    }
    fn can_build(&self, t: usize, b: &Blueprint) -> bool {
        self.can_build_n(t, b, 1)
    }
    fn can_build_anything(&self, b: &Blueprint) -> bool {
        for t in ALL_TYPES {
            if self.can_build(t, b) {
                return true;
            }
        }
        false
    }
    fn next_states(&self, b: &Blueprint, bc: &BestCase) -> Vec<State> {
        let mut states = Vec::new();
        if self.tr > 1 {
            // This is wrong for part 2 of the simple example. So apparently there are cases where
            // you should WAIT to build a critical-path robot.
            // for t in ALL_TYPES {
            //     if self.robots[t] == 0 && self.can_build(t, b) {
            //         // println!("Must build {:?}", t);
            //         states.push_back(self.build_robot(t, b));
            //         break;
            //     }
            // }
            if states.is_empty() {
                states.push(self.do_nothing());
                for t in ALL_TYPES {
                    if self.can_build(t, b) {
                        // println!("Can build {:?}", t);
                        states.push(self.build_robot(t, b));
                    }
                }
            }
        }
        if states.is_empty() && self.tr > 0 {
            states.push(self.do_nothing());
        }
        for s in &mut states {
            s.geode_lower_limit = s.predict_resource(GEODE);
            s.geode_upper_limit = s.geode_upper_bound(bc);
            if s.geode_lower_limit > s.geode_upper_limit {
                panic!()
            }
        }
        // println!("{} next options", states.len());
        states
    }
    fn do_nothing(&self) -> State {
        let mut s = self.clone();
        s.time += 1;
        s.tr -= 1;
        for t in ALL_TYPES {
            s.resources[t] += s.robots[t];
        }
        s
    }
    fn build_robot(&self, t: usize, b: &Blueprint) -> State {
        let mut s = self.do_nothing();
        s.robots[t] += 1;
        match t {
            GEODE => {
                s.resources[ORE] -= b.geode_ore_cost;
                s.resources[OBSIDIAN] -= b.geode_obs_cost;
            }
            OBSIDIAN => {
                s.resources[ORE] -= b.obs_ore_cost;
                s.resources[CLAY] -= b.obs_clay_cost;
            }
            CLAY => {
                s.resources[ORE] -= b.clay_cost;
            }
            ORE => {
                s.resources[ORE] -= b.ore_cost;
            }
            _ => panic!(),
        }
        s
    }
    fn predict_resource(&self, t: usize) -> i32 {
        self.resources[t] + self.robots[t] * self.tr
    }
    fn geode_upper_bound(&self, bc: &BestCase) -> i32 {
        std::cmp::min(
            self.predict_resource(GEODE) + self.tr * (self.tr - 1) / 2,
            bc.most_geodes(self),
        )
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|{:9}|",
            self.time,
            self.tr,
            self.geode_lower_limit,
            self.geode_upper_limit,
            self.resources[GEODE],
            self.robots[GEODE],
            self.resources[OBSIDIAN],
            self.robots[OBSIDIAN],
            self.resources[CLAY],
            self.robots[CLAY],
            self.resources[ORE],
            self.robots[ORE],
        )
    }
}

struct BestCase {
    data: Vec<State>,
    first_robot: [i32; 4],
}
impl BestCase {
    fn from(b: &Blueprint, max_t: i32) -> BestCase {
        // State::print_header();
        let mut best_case = vec![State::new(max_t)];
        let mut first_robot: [i32; 4] = [0, 0, 0, 0];
        // println!("{}", best_case.last().unwrap());
        for time in 1..=max_t {
            let last = best_case.last().unwrap();
            let mut s = best_case.last().unwrap().do_nothing();
            for t in ALL_TYPES {
                if last.can_build_n(t, b, s.robots[t] - best_case.first().unwrap().robots[t] + 1) {
                    s.robots[t] += 1;
                    if s.robots[t] == 1 {
                        first_robot[t] = time;
                    }
                }
            }
            // println!("{}", s);
            best_case.push(s);
        }
        BestCase {
            data: best_case,
            first_robot: first_robot,
        }
    }
    fn most_geodes(&self, s: &State) -> i32 {
        for t in [CLAY, OBSIDIAN, GEODE] {
            if s.robots[t] == 0 {
                // println!("{} {} {}", self.first_robot[t], s.tr, self.first_robot[t] - 1 + s.tr);
                return self.data
                    [std::cmp::min(self.data.len() - 1, (self.first_robot[t] + s.tr) as usize)]
                .resources[GEODE];
            }
        }
        for time in self.data.len() - 1..=0 {
            if s.robots[GEODE] == self.data[time].robots[GEODE] {
                return self.data[std::cmp::min(self.data.len() - 1, time + s.tr as usize)].robots
                    [GEODE];
            }
        }
        self.data.last().unwrap().resources[GEODE]
    }
    fn print(&self) {
        State::print_header();
        for s in &self.data {
            println!("{}", s);
        }
        println!("{:?}", self.first_robot);
    }
}

fn main() {
    let mut bs = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        bs.push(Blueprint {
            id: tokens[1].strip_suffix(":").unwrap().parse::<i32>().unwrap(),
            ore_cost: tokens[6].parse::<i32>().unwrap(),
            clay_cost: tokens[12].parse::<i32>().unwrap(),
            obs_ore_cost: tokens[18].parse::<i32>().unwrap(),
            obs_clay_cost: tokens[21].parse::<i32>().unwrap(),
            geode_ore_cost: tokens[27].parse::<i32>().unwrap(),
            geode_obs_cost: tokens[30].parse::<i32>().unwrap(),
        });
    }

    let mut quality = 0;
    for b in &bs {
        let most_geodes = b.find_best(24);
        println!("Blueprint {}, Geodes {}", b.id, most_geodes);
        quality += b.id * most_geodes;
    }
    println!("Part 1: {}", quality);

    let mut prod = 1;
    for i in 0..3 {
        let most_geodes = bs[i].find_best(32);
        println!("Blueprint {}, Geodes {}", bs[i].id, most_geodes);
        prod *= most_geodes;
    }
    println!("Part 2: {}", prod);
}
