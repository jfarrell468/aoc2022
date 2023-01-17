use core::fmt;
use std::collections::{HashSet, HashMap};
use std::cmp;

#[derive(Debug, Clone)]
struct Map {
    elves: HashSet<(i32, i32)>,
    rmin: i32,
    rmax: i32,
    cmin: i32,
    cmax: i32,
}
impl Map {
    fn parse() -> Map {
        let map = std::io::stdin().lines().map(|l| l.unwrap().chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut elves = HashSet::new();
        let mut rmin = 0;
        let mut rmax = 0;
        let mut cmin = 0;
        let mut cmax = 0;
        for r in 0..map.len() as i32 {
            for c in 0..map[0].len() as i32 {
                if map[r as usize][c as usize] {
                    if elves.is_empty() {
                        rmin = r;
                        rmax = r;
                        cmin = c;
                        cmax = c;
                    } else {
                        rmin = std::cmp::min(r, rmin);
                        rmax = std::cmp::max(r, rmax);
                        cmin = std::cmp::min(c, cmin);
                        cmax = std::cmp::max(c, cmax);
                    }
                    elves.insert((r, c));
                }
            }
        }
        Map {
            elves, rmin, rmax, cmin, cmax
        }
    }
    fn has_neighbor(&self, pos: &(i32, i32)) -> bool {
        assert!(self.elves.contains(pos));
        for delta in [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)] {
            if self.elves.contains(&(pos.0 + delta.0, pos.1 + delta.1)) {
                return true;
            }
        }
        false
    }
    fn print_with_annotation(&self, pos: &(i32, i32)) {
        for r in self.rmin..=self.rmax {
            for c in self.cmin..=self.cmax {
                print!("{}", if r == pos.0 && c == pos.1 { "@" } else if self.elves.contains(&(r, c)) { "#" } else { "." });
            }
            // write!(f, "  {}\n", r)?;
            println!("")
        }
    }
    fn execute(&mut self, max_rounds: Option<usize>) {
        // println!("Starting position");
        // println!("{}", self);
        for round in 0..max_rounds.unwrap_or(std::usize::MAX) {
            let mut proposals = HashMap::new();
            let mut invalid_proposals = HashSet::new();
            for elf in &self.elves {
                // println!("");
                // self.print_with_annotation(elf);
                if !self.has_neighbor(elf) {
                    // println!("Has no neighbors");
                    continue;
                }
                for rr in round..(round + 4) {
                    let (delta, check) = [
                        ((-1, 0), [(-1, -1), (-1, 0), (-1, 1)]),
                        ((1, 0), [(1, -1), (1, 0), (1, 1)]), 
                        ((0, -1), [(-1, -1), (0, -1), (1, -1)]), 
                        ((0, 1), [(-1, 1), (0, 1), (1, 1)])
                    ][rr % 4];
                    let mut all_empty = true;
                    let to = (elf.0 + delta.0, elf.1 + delta.1);
                    for c in check {
                        if self.elves.contains(&(elf.0 + c.0, elf.1 + c.1)) {
                            all_empty = false;
                            break;
                        }
                    }
                    if all_empty {
                        // println!("Can move in direction {:?}", delta);
                        if !invalid_proposals.contains(&to) {
                            // println!("No conflict at {:?}", to);
                            assert!(proposals.insert(to.clone(), (elf.0, elf.1)).is_none());
                            assert!(invalid_proposals.insert(to));
                        } else {
                            // println!("Conflict at {:?}", to);
                            proposals.remove(&to);
                        }
                        break;
                    } else {
                        // println!("Can't move in direction {:?}", delta);
                    }
                }
            }
            if proposals.is_empty() {
                println!("No changes after {} rounds", round + 1);
                break;
            }
            for (to, from) in proposals {
                // println!("Move from {:?} to {:?}", from, to);
                assert!(self.elves.remove(&from), "No elf at {:?}", from);
                assert!(self.elves.insert(to), "Unexpected elf at {:?}", to);
            }
            let e1 = self.elves.iter().next().unwrap();
            self.rmin = e1.0;
            self.rmax = e1.0;
            self.cmin = e1.1;
            self.cmin = e1.1;
            for elf in &self.elves {
                self.rmin = std::cmp::min(elf.0, self.rmin);
                self.rmax = std::cmp::max(elf.0, self.rmax);
                self.cmin = std::cmp::min(elf.1, self.cmin);
                self.cmax = std::cmp::max(elf.1, self.cmax);
            }
            // println!("Round {}", round + 1);
            // println!("{}", self)
        }
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in self.rmin..=self.rmax {
            for c in self.cmin..=self.cmax {
                write!(f, "{}", if self.elves.contains(&(r, c)) { "#" } else { "." })?;
            }
            // write!(f, "  {}\n", r)?;
            write!(f, "\n")?;
        }
        Ok(())
    }
}


fn main() {
    let mut m = Map::parse();
    let mut m2 = m.clone();
    m.execute(Some(10));
    println!("Part 1: {}", (m.rmax - m.rmin + 1)*(m.cmax - m.cmin + 1) - m.elves.len() as i32);
    m2.execute(None);
}
