use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
enum Contents {
    Empty,
    FallingRock,
    FixedRock,
}

const CHAMBER_WIDTH: u8 = 7;

#[derive(Debug, Clone)]
struct Rock {
    r: u64,  // Assumes little-endian.
    w: u8,
    h: u8,
    pos: (u8, usize),
}
impl Rock {
    fn parse(s: &str) -> Rock {
        let mut rows: Vec<&str> = s.split(",").collect();
        rows.reverse();
        let mut rock = Rock {
            r: 0,
            w: 0,
            h: rows.len().try_into().unwrap(),
            pos: (0, 0),
        };
        for y in 0..rows.len() {
            let row: Vec<bool> = rows[y].chars().map(|e| e == '#').collect();
            rock.w = row.len().try_into().unwrap();
            for x in 0..row.len() {
                if row[x] {
                    rock.r |= 1 << (8 * y + x);
                }
            }
        }
        rock
    }
    fn print(&self) {
        for y in (0..self.h as u64).rev() {
            for x in 0..self.w {
                print!(
                    "{}",
                    if self.r & 1 << (x as u64 + 8 * y) == 0 {
                        '.'
                    } else {
                        '#'
                    }
                );
            }
            println!("");
        }
    }
    fn at(&self, pos: (u8, usize)) -> Contents {
        if pos.0 < self.pos.0
            || pos.0 > self.pos.0 + self.w - 1
            || pos.1 < self.pos.1
            || pos.1 > self.pos.1 + self.h as usize - 1
            || self.r & 1 << (8 * ((pos.1 - self.pos.1) as u64) + (pos.0 - self.pos.0) as u64) == 0
        {
            return Contents::Empty;
        }
        Contents::FallingRock
    }
    fn can_move(&self, d: Dir) -> bool {
        match d {
            Dir::Left => self.pos.0 > 0,
            Dir::Right => self.pos.0 + self.w < CHAMBER_WIDTH,
            Dir::Down => self.pos.1 > 0,
        }
    }
    fn mv(&mut self, d: Dir) {
        assert!(self.can_move(d));
        match d {
            Dir::Left => self.pos.0 -= 1,
            Dir::Right => self.pos.0 += 1,
            Dir::Down => self.pos.1 -= 1,
        }
    }
}

struct Chamber {
    r: Vec<u8>,
    t: usize,
    jets: Vec<Dir>,
}
impl Chamber {
    fn new(jets: Vec<Dir>) -> Self {
        Chamber {
            r: vec![0; 16],
            t: 0,
            jets,
        }
    }
    fn print(&self, rock: Option<&Rock>) {
        for y in (0..self.height(rock.clone())).rev() {
            print!("|");
            for x in 0..CHAMBER_WIDTH {
                print!(
                    "{}",
                    match self.at((x, y), rock) {
                        Contents::Empty => '.',
                        Contents::FallingRock => '@',
                        Contents::FixedRock => '#',
                    }
                );
            }
            println!("|");
        }
        println!("+-------+");
    }
    fn drop_rock(&mut self, mut rock: Rock) {
        rock.pos = (2, self.r.len() + 3 - 16);
        // self.print(Some(&rock));
        // println!("");
        loop {
            let jet_dir = self.jets[self.t % self.jets.len()];
            if self.can_move(&rock, jet_dir) {
                rock.mv(jet_dir);
            }
            // self.print(Some(&rock));
            // println!("");
            self.t += 1;
            if !self.can_move(&rock, Dir::Down) {
                break;
            }
            rock.mv(Dir::Down);
            // self.print(Some(&rock));
            // println!("");
        }
        while self.r.len() < self.height(Some(&rock)) + 16 {
            self.r.push(0);
        }
        let mut r;
        unsafe {
            // Safe because we always maintain 16 extra elements at the end of the vector.
            r = &mut *std::mem::transmute::<*mut u8, *mut u64>(&mut self.r[rock.pos.1] as *mut u8);
        }
        *r |= rock.r << rock.pos.0;

        // self.print(None);
    }
    fn can_move(&self, rock: &Rock, d: Dir) -> bool {
        if !rock.can_move(d) {
            return false;
        }
        let shifted_rock = match d {
            Dir::Left => rock.r << (rock.pos.0 - 1),
            Dir::Right => rock.r << (rock.pos.0 + 1),
            Dir::Down => rock.r << rock.pos.0,
        };
        let chamber_idx = match d {
            Dir::Left => rock.pos.1,
            Dir::Right => rock.pos.1,
            Dir::Down => rock.pos.1 - 1,
        };
        let r;
        unsafe {
            // Safe because we always maintain 16 extra elements at the end of the vector.
            r = *std::mem::transmute::<*const u8, *const u64>(&self.r[chamber_idx] as *const u8);
        }

        return r & shifted_rock == 0;
    }
    fn at(&self, pos: (u8, usize), rock: Option<&Rock>) -> Contents {
        match rock {
            Some(r) => match r.at(pos) {
                Contents::Empty => self.at(pos, None),
                Contents::FallingRock => Contents::FallingRock,
                Contents::FixedRock => panic!(),
            },
            None => {
                if pos.1 >= self.r.len() || self.r[pos.1] & 1 << pos.0 == 0 {
                    Contents::Empty
                } else {
                    Contents::FixedRock
                }
            }
        }
    }
    fn height(&self, rock: Option<&Rock>) -> usize {
        match rock {
            Some(r) => std::cmp::max(self.r.len() - 16, r.pos.1 + r.h as usize),
            None => self.r.len() - 16,
        }
    }
}

fn main() {
    let mut gas_jets = Vec::new();
    for c in std::io::stdin().lines().next().unwrap().unwrap().chars() {
        gas_jets.push(match c {
            '>' => Dir::Right,
            '<' => Dir::Left,
            _ => panic!(),
        });
    }
    // println!("{:?}", gas_jets);

    let mut rocks = vec![
        Rock::parse("####"),
        Rock::parse(".#.,###,.#."),
        Rock::parse("..#,..#,###"),
        Rock::parse("#,#,#,#"),
        Rock::parse("##,##"),
    ];
    println!(
        "{} gas jets, {} rocks, product = {}",
        gas_jets.len(),
        rocks.len(),
        gas_jets.len() * rocks.len()
    );
    // for rock in &rocks {
    //     println!("{:?}", rock);
    //     rock.print();
    //     println!("");
    // }
    // return;

    // let mut chamber = Chamber::new(gas_jets.clone());
    // for r in 0..20 {
    //     println!("Dropping rock {}", r);
    //     chamber.drop_rock(rocks[r % rocks.len()].clone());
    // }
    // return;

    let mut chamber = Chamber::new(gas_jets.clone());
    for r in 0..2022 {
        // println!("Dropping rock {}", r);
        chamber.drop_rock(rocks[r % rocks.len()].clone());
    }
    println!("Part 1: {}", chamber.r.len() - 16);

    let mut chamber = Chamber::new(gas_jets.clone());
    let mut full_rows = HashMap::new();
    let mut first_full_row_rocks = 0;
    let mut height_per_cycle = 0;
    let mut num_cycles = 0;
    let mut rocks_at_end = 0;
    let max_rocks: usize = 1000000000000;
    for r in 0..10000 {
        chamber.drop_rock(rocks[r % rocks.len()].clone());
        if chamber.r[chamber.r.len() - 17] == 127 {
            // A full row is 0b01111111 = 127
            let rock_idx = r % rocks.len();
            let jet_idx = chamber.t % chamber.jets.len();
            // chamber.print(None);
            // println!("full row after {} rocks", r + 1);
            if full_rows.contains_key(&(rock_idx, jet_idx)) {
                println!(
                    "Found duplicate full row with rock idx {}, jet idx {}",
                    rock_idx, jet_idx
                );
                println!(
                    "Current state: {} rocks, {} height",
                    r + 1,
                    chamber.height(None)
                );
                let first_full_row_height;
                (first_full_row_rocks, first_full_row_height) =
                    full_rows[&(r % rocks.len(), chamber.t % chamber.jets.len())];
                println!(
                    "Previous state state: {} rocks, {} height",
                    first_full_row_rocks, first_full_row_height
                );
                let rocks_per_cycle = r + 1 - first_full_row_rocks;
                height_per_cycle = chamber.height(None) - first_full_row_height;
                println!(
                    "Cycle length: {} rocks, {} height",
                    rocks_per_cycle, height_per_cycle
                );
                num_cycles = (max_rocks - first_full_row_rocks) / rocks_per_cycle;
                rocks_at_end = max_rocks - first_full_row_rocks - rocks_per_cycle * num_cycles;
                println!(
                    "{} rocks has {} full cycles with {} rocks before and {} rocks after",
                    max_rocks, num_cycles, first_full_row_rocks, rocks_at_end
                );
                println!(
                    "Check: {} + {} * {} + {} == {} (expected) {} (actual)",
                    first_full_row_rocks,
                    num_cycles,
                    rocks_per_cycle,
                    rocks_at_end,
                    max_rocks,
                    first_full_row_rocks + num_cycles * rocks_per_cycle + rocks_at_end
                );
                break;
            } else {
                full_rows.insert((rock_idx, jet_idx), (r + 1, chamber.height(None)));
            }
        }
    }
    let mut chamber = Chamber::new(gas_jets.clone());
    for r in 0..(first_full_row_rocks + rocks_at_end) {
        chamber.drop_rock(rocks[r % rocks.len()].clone());
    }
    println!(
        "Part 2: {}",
        chamber.height(None) + num_cycles * height_per_cycle
    );
}
