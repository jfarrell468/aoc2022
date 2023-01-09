/*

* There are never any up/down blizzards in the first/last column that could "escape".
* 

*/

use std::collections::{BinaryHeap, VecDeque};

const STAY: u8 = 0;
const UP: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;
const RIGHT: u8 = 4;

#[derive(Clone)]
struct Valley {
    map: Vec<Vec<Vec<u8>>>,
}
impl Valley {
    fn read_from_stdin() -> Valley {
        let mut rows = Vec::new();
        for line in std::io::stdin().lines() {
            let mut row = Vec::new();
            for c in line.unwrap().chars() {
                let mut blizzards = Vec::new();
                match c {
                    '^' => { blizzards.push(UP); },
                    'v' => { blizzards.push(DOWN); },
                    '<' => { blizzards.push(LEFT); },
                    '>' => { blizzards.push(RIGHT); },
                    '#' | '.' => {},
                    _ => panic!(),
                }
                row.push(blizzards);
            }
            rows.push(row);
        };
        Valley {
            map: rows
        }
    }
    fn print(&self) {
        for r in 0..self.map.len() {
            for c in 0..self.map[r].len() {
                if self.is_wall(r, c) {
                    // if r == 0 && c == 1 || r == self.map.len() - 1 && c == self.map[0].len() - 2 {
                    //     print!(".")
                    // } else {
                        print!("#");
                    // }
                } else if self.map[r][c].is_empty() {
                    print!(".");
                } else if self.map[r][c].len() == 1 {
                    print!("{}", match *self.map[r][c].last().unwrap() {
                        UP => '^',
                        DOWN => 'v',
                        LEFT => '<',
                        RIGHT => '>',
                        _ => panic!()
                    });
                } else {
                    print!("{}", self.map[r][c].len())
                }
            }
            println!("");
        }
    }
    fn is_wall(&self, r: usize, c: usize) -> bool {
        c == 0 || c == self.map[0].len() - 1 || r == 0 && c != 1 || r == self.map.len() - 1 && c != self.map[0].len() - 2
    }
    fn advance(&mut self) {
        let mut next_v = Valley {
            map: vec![vec![vec![]; self.map[0].len()]; self.map.len()]
        };
        for r in 0..self.map.len() {
            for c in 0..self.map[r].len() {
                for b in &self.map[r][c] {
                    let (mut next_r, mut next_c) = match *b as u8 {
                        UP => (if r == 1 { self.map.len() - 2 } else { r - 1 }, c),
                        DOWN => (if r == self.map.len() - 2 { 1 } else { r + 1 }, c),
                        LEFT => (r, if c == 1 { self.map[0].len() - 2 } else { c - 1 }),
                        RIGHT => (r, if c == self.map[0].len() - 2 { 1 } else {c + 1 }),
                        _ => panic!()
                    };
                    next_v.map[next_r][next_c].push(*b);
                }
            }
        }
        *self = next_v;
    }
}

struct ValleyState {
    open: Vec<Vec<Vec<bool>>>,
    nr: usize,
    nc: usize,
}
impl ValleyState {
    fn from(v: &Valley) -> ValleyState {
        let mut vs = ValleyState {
            open: vec![vec![vec![false; v.map[0].len()]; v.map.len()]; v.map.len() * v.map[0].len()],
            nr: v.map.len(),
            nc: v.map[0].len()
        };
        let mut v = v.clone();
        for t in 0..vs.nr * vs.nc {
            for r in 0..vs.nr {
                for c in 0..vs.nc {
                    vs.open[t][r][c] = v.map[r][c].is_empty() && !v.is_wall(r, c)
                }
            }
            v.advance();
        }
        vs
    }
    fn is_open(&self, t: usize, r: usize, c: usize) -> bool {
        r < self.nr && c < self.nc && self.open[t % self.open.len()][r][c]
    }
    fn print(&self, t: usize) {
        for r in 0..self.nr {
            for c in 0..self.nc {
                if self.is_open(t, r, c) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }
    fn distance_to_goal(&self, r: usize, c: usize) -> usize {
        self.nr - r + self.nc - c - 3
    }
    fn print_with_explorer(&self, e: &Explorer) {
        for r in 0..self.nr {
            for c in 0..self.nc {
                if r == e.r && c == e.c {
                    print!("E");
                } else if self.is_open(e.t, r, c) {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }
}

struct Reachability {
    reachable: Vec<Vec<Vec<bool>>>,
    nr: usize,
    nc: usize,
}
impl Reachability {
    fn compute_from(vs: &ValleyState, max_t: usize, start_t: usize, start: (usize, usize), goal: (usize, usize)) -> usize {
        let mut rr = Reachability {
            reachable: vec![vec![vec![false; vs.nc]; vs.nr]; max_t+1],
            nr: vs.nr,
            nc: vs.nc
        };
        rr.reachable[start_t][start.0][start.1] = true;
        for t in start_t..max_t {
            for r in 0..rr.nr {
                for c in 0..rr.nc {
                    if rr.reachable[t][r][c] {
                        for (rn, cn) in [(r, c), (r-1, c), (r, c-1), (r+1, c), (r, c+1)] {
                            if vs.is_open(t+1, rn, cn) {
                                rr.reachable[t+1][rn][cn] = true;
                            }
                        }
                    }
                }
            }
            if rr.reachable[t][goal.0][goal.1] {
                return t;
            }
        }
        0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Explorer {
    h: i32,  // Heuristic. Negative of manhattan distance to goal, since BinaryHeap is largest first.
    t: usize,
    r: usize,
    c: usize,
}
impl Explorer {
    fn new(vs: &ValleyState) -> Explorer {
        Explorer {
            h: -(vs.distance_to_goal(0, 1) as i32),
            t: 0,
            r: 0,
            c: 1,
        }
    }
    fn expand(&self, vs: &ValleyState) -> VecDeque<Explorer> {
        let mut neighbors = VecDeque::new();
        for (r, c) in [(self.r, self.c), (self.r-1, self.c), (self.r, self.c-1), (self.r+1, self.c), (self.r, self.c+1)] {
            if vs.is_open(self.t+1, r, c) {
                neighbors.push_back(Explorer {
                    h: -((vs.distance_to_goal(r, c) + self.t + 1) as i32),
                    t: self.t + 1,
                    r,
                    c,
                })
            }
        }
        neighbors
    }
    fn at_goal(&self, vs: &ValleyState) -> bool {
        self.r == vs.nr - 1 && self.c == vs.nc - 2
    }
}

fn main() {
    let mut v = Valley::read_from_stdin();
    let vs = ValleyState::from(&v);

    println!("Part 1: {}", Reachability::compute_from(&vs, 1000, 0, (0,1), (vs.nr-1, vs.nc-2)));

    let t1 = Reachability::compute_from(&vs, 10000, 0, (0,1), (vs.nr-1, vs.nc-2));
    let t2 = Reachability::compute_from(&vs, 10000, t1, (vs.nr-1, vs.nc-2), (0,1));
    let t3 = Reachability::compute_from(&vs, 10000, t2, (0,1), (vs.nr-1, vs.nc-2));
    println!("Part 2: {}", t3);
}
