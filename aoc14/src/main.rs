#[derive(Clone)]
enum Contents {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug)]
enum DropOutcome {
    Escaped,
    Position((usize, usize)),
}

struct Cave {
    c: Vec<Vec<Contents>>,
    xmin: usize,
    xmax: usize,
    ymax: usize,
}
impl Cave {
    fn read_from_stdin() -> Cave {
        let mut rocks = Vec::new();
        let mut xmin = 0;
        let mut xmax = 0;
        let mut ymax = 0;
        for line in std::io::stdin().lines() {
            let line = line.unwrap();
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let mut cur_scan = Vec::new();
            for t in tokens {
                if t == "->" {
                    continue;
                }
                let (x, y) = t.split_once(",").unwrap();
                let x = x.parse::<usize>().unwrap();
                let y = y.parse::<usize>().unwrap();
                cur_scan.push((x, y));
                if xmin == 0 || x < xmin {
                    xmin = x;
                }
                if x > xmax {
                    xmax = x;
                }
                if y > ymax {
                    ymax = y;
                }
            }
            rocks.push(cur_scan);
        }
        // println!("{:?}", rocks);
        // println!("xmin = {}, xmax = {}, ymax = {}", xmin, xmax, ymax);
        let mut cave = Cave {
            c: vec![vec![Contents::Empty; (xmax - xmin + 1) as usize]; (ymax + 1) as usize],
            xmin,
            xmax,
            ymax,
        };
        for r in 0..rocks.len() {
            for c in 1..rocks[r].len() {
                let from = rocks[r][c - 1];
                let to = rocks[r][c];
                // println!("from {:?} to {:?}", from, to);
                for xx in std::cmp::min(from.0, to.0)..=std::cmp::max(from.0, to.0) {
                    // println!("{}, {}", xx, from.1);
                    cave.set((xx, from.1), Contents::Rock);
                    // cave[from.1 as usize][(xx - xmin) as usize] = Contents::Rock;
                }
                for yy in std::cmp::min(from.1, to.1)..=std::cmp::max(from.1, to.1) {
                    // println!("{}, {}", from.0, yy);
                    cave.set((from.0, yy), Contents::Rock);
                    // cave[yy as usize][(from.0 - xmin) as usize] = Contents::Rock;
                }
            }
        }
        cave
    }
    fn print(&self) {
        for x in &self.c {
            for y in x {
                print!(
                    "{}",
                    match y {
                        Contents::Empty => ".",
                        Contents::Rock => "#",
                        Contents::Sand => "o",
                    }
                );
            }
            println!("")
        }
    }
    fn drop(&mut self, pos: (usize, usize)) -> DropOutcome {
        // println!("drop {:?}", pos);
        if self.escaped(pos) {
            // println!("escaped");
            return DropOutcome::Escaped;
        }
        for t in [
            (pos.0, pos.1 + 1),
            (pos.0 - 1, pos.1 + 1),
            (pos.0 + 1, pos.1 + 1),
        ] {
            // println!("trying {:?}", t);
            if self.escaped(t) {
                // println!("escaped");
                return DropOutcome::Escaped;
            } else if let Contents::Empty = self.get(t) {
                // println!("empty at {:?}", t);
                return self.drop(t);
            }
        }
        // println!("final position {:?}", pos);
        self.set(pos, Contents::Sand);
        DropOutcome::Position(pos)
    }
    fn get(&self, pos: (usize, usize)) -> &Contents {
        if pos.1 > self.ymax || pos.0 > self.xmax || pos.0 < self.xmin {
            &self.c[0][0]
        } else {
            &self.c[pos.1 as usize][(pos.0 - self.xmin) as usize]
        }
    }
    fn set(&mut self, pos: (usize, usize), c: Contents) {
        self.c[pos.1 as usize][(pos.0 - self.xmin) as usize] = c;
    }
    fn escaped(&self, pos: (usize, usize)) -> bool {
        pos.1 > self.ymax
    }
}

fn main() {
    let mut c = Cave::read_from_stdin();
    // c.print();
    let mut t = 0;
    while let DropOutcome::Position(_) = c.drop((500, 0)) {
        // c.print();
        t += 1;
    }
    c.print();
    println!("Part 1: {}", t)
}
