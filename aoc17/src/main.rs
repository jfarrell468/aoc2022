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

const CHAMBER_WIDTH: usize = 7;

#[derive(Debug, Clone)]
struct Rock {
    r: Vec<Vec<bool>>,
    pos: (usize, usize),
}
impl Rock {
    fn parse(s: &str) -> Rock {
        let mut rock = Rock {
            r: Vec::new(),
            pos: (0, 0),
        };
        for row in s.split(",") {
            rock.r.push(row.chars().map(|e| e == '#').collect());
        }
        rock.r.reverse();
        rock
    }
    fn print(&self) {
        for y in self.r.iter().rev() {
            println!(
                "{}",
                y.iter()
                    .map(|e| if *e { "#".to_string() } else { ".".to_string() })
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
    }
    fn at(&self, pos: (usize, usize)) -> Contents {
        if pos.0 < self.pos.0
            || pos.0 > self.pos.0 + self.r[0].len() - 1
            || pos.1 < self.pos.1
            || pos.1 > self.pos.1 + self.r.len() - 1
            || !self.r[pos.1 - self.pos.1][pos.0 - self.pos.0]
        {
            return Contents::Empty;
        }
        Contents::FallingRock
    }
    fn can_move(&self, d: Dir) -> bool {
        match d {
            Dir::Left => self.pos.0 > 0,
            Dir::Right => self.pos.0 + self.r[0].len() < CHAMBER_WIDTH,
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
    r: Vec<[bool; CHAMBER_WIDTH]>,
    t: usize,
    jets: Vec<Dir>,
}
impl Chamber {
    fn new(jets: Vec<Dir>) -> Self {
        Chamber {
            r: Vec::new(),
            t: 0,
            jets,
        }
    }
    fn print(&self, rock: Option<&Rock>) {
        for y in (0..self.ymax(rock.clone())).rev() {
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
        rock.pos = (2, self.r.len() + 3);
        // self.print(Some(&rock));
        // println!("");
        loop {
            let jet_dir = self.jets[self.t % self.jets.len()];
            if self.can_move(&rock, jet_dir) {
                rock.mv(jet_dir);
            }
            self.t += 1;
            if !self.can_move(&rock, Dir::Down) {
                break;
            }
            rock.mv(Dir::Down);
        }
        while self.r.len() < self.ymax(Some(&rock)) {
            self.r.push([false; CHAMBER_WIDTH]);
        }
        for y in rock.pos.1..rock.pos.1 + rock.r.len() {
            for x in rock.pos.0..rock.pos.0 + rock.r[0].len() {
                if let Contents::FallingRock = rock.at((x, y)) {
                    assert!(self.at((x, y), None) == Contents::Empty);
                    self.r[y][x] = true;
                }
            }
        }
        // self.print(None);
    }
    fn can_move(&self, rock: &Rock, d: Dir) -> bool {
        if !rock.can_move(d) {
            return false;
        }
        for y in rock.pos.1..rock.pos.1 + rock.r.len() {
            for x in rock.pos.0..rock.pos.0 + rock.r[0].len() {
                if let Contents::FallingRock = rock.at((x, y)) {
                    if let Contents::FixedRock = self.at(
                        match d {
                            Dir::Left => (x - 1, y),
                            Dir::Right => (x + 1, y),
                            Dir::Down => (x, y - 1),
                        },
                        Some(rock),
                    ) {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn at(&self, pos: (usize, usize), rock: Option<&Rock>) -> Contents {
        match rock {
            Some(r) => match r.at(pos) {
                Contents::Empty => self.at(pos, None),
                Contents::FallingRock => Contents::FallingRock,
                Contents::FixedRock => panic!(),
            },
            None => {
                if pos.1 >= self.r.len() || !self.r[pos.1][pos.0] {
                    Contents::Empty
                } else {
                    Contents::FixedRock
                }
            }
        }
    }
    fn ymax(&self, rock: Option<&Rock>) -> usize {
        match rock {
            Some(r) => std::cmp::max(self.r.len(), r.pos.1 + r.r.len()),
            None => self.r.len(),
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

    let mut chamber = Chamber::new(gas_jets);
    for r in 0..2022 {
        // println!("Dropping rock {}", r);
        chamber.drop_rock(rocks[r % rocks.len()].clone());
    }
    println!("Part 1: {}", chamber.r.len());
}
