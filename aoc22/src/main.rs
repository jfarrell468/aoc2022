use std::fmt;

#[derive(Debug, Clone)]
enum Contents {
    Invalid,
    Empty,
    Wall,
}
impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Contents::Invalid => " ",
                Contents::Empty => ".",
                Contents::Wall => "#",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Maze {
    m: Vec<Vec<Contents>>,
}
impl Maze {
    fn parse(lines: &Vec<String>) -> Maze {
        let mut m = Vec::new();
        let mut max_length = 0;
        for line in lines {
            let chars = line.chars().collect::<Vec<_>>();
            max_length = std::cmp::max(max_length, chars.len());
            let mut lv = Vec::new();
            for c in chars {
                lv.push(match c {
                    ' ' => Contents::Invalid,
                    '.' => Contents::Empty,
                    '#' => Contents::Wall,
                    _ => panic!(),
                });
            }
            m.push(lv);
        }
        for r in &mut m {
            while r.len() < max_length {
                r.push(Contents::Invalid);
            }
        }
        Maze { m }
    }
}
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in &self.m {
            for c in r {
                write!(f, "{}", c)?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Command {
    Forward(i32),
    RotateLeft,
    RotateRight,
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::RotateLeft => write!(f, "L"),
            Command::RotateRight => write!(f, "R"),
            Command::Forward(d) => write!(f, "{}", d),
        }
    }
}

struct Commands {
    c: Vec<Command>,
}
impl Commands {
    fn parse(s: &str) -> Commands {
        let mut c = Vec::new();
        for item in s.split_inclusive(&['L', 'R']) {
            if let Some(d) = item.strip_suffix('L') {
                c.push(Command::Forward(d.parse::<i32>().unwrap()));
                c.push(Command::RotateLeft);
            } else if let Some(d) = item.strip_suffix('R') {
                c.push(Command::Forward(d.parse::<i32>().unwrap()));
                c.push(Command::RotateRight);
            } else {
                c.push(Command::Forward(item.parse::<i32>().unwrap()));
            }
        }
        Commands { c }
    }
}
impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.c {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    fn to_score(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
    fn rotate(&mut self, c: &Command) {
        match c {
            Command::RotateLeft => {
                *self = match self {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                }
            }
            Command::RotateRight => {
                *self = match self {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
            Command::Forward(_) => panic!(),
        };
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Left => "<",
                Direction::Right => ">",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Turtle {
    d: Direction,
    r: usize,
    c: usize,
}
impl Turtle {
    fn new(m: &Maze) -> Turtle {
        let mut c = 0;
        for tile in &m.m[0] {
            if let Contents::Invalid = tile {
                c += 1;
            } else {
                break;
            }
        }
        Turtle {
            d: Direction::Right,
            r: 0,
            c: c,
        }
    }
    fn offset_tile(&self, m: &Maze, d: usize) -> (usize, usize) {
        match self.d {
            Direction::Right => (self.r, (self.c + d) % m.m[0].len()),
            Direction::Down => ((self.r + d) % m.m.len(), self.c),
            Direction::Left => (self.r, (self.c + m.m[0].len() - d) % m.m[0].len()),
            Direction::Up => ((self.r + m.m.len() - d) % m.m.len(), self.c),
        }
    }
    fn execute(&mut self, c: &Command, m: &Maze) {
        match c {
            Command::Forward(dist) => {
                let mut offset = 0;
                let mut last_valid_offset = 0;
                let mut moved = 0;
                while moved < *dist {
                    let (row, col) = self.offset_tile(m, offset + 1);
                    match m.m[row][col] {
                        Contents::Invalid => {
                            offset += 1;
                        }
                        Contents::Empty => {
                            moved += 1;
                            offset += 1;
                            last_valid_offset = offset;
                        }
                        Contents::Wall => {
                            break;
                        }
                    }
                }
                (self.r, self.c) = self.offset_tile(m, last_valid_offset);
            }
            Command::RotateLeft => self.d.rotate(c),
            Command::RotateRight => self.d.rotate(c),
        }
    }
    fn password(&self) -> usize {
        1000 * (self.r + 1) + 4 * (self.c + 1) + self.d.to_score()
    }
}

struct Traversal {
    m: Maze,
    c: Commands,
    t: Turtle,
}
impl Traversal {
    fn new(m: Maze, c: Commands) -> Traversal {
        let t = Turtle::new(&m);
        Traversal { m, c, t }
    }
    fn execute(&mut self) {
        for c in &self.c.c {
            // println!("Executing {:?}", c);
            self.t.execute(c, &self.m);
            // println!("{}", self);
        }
    }
}
impl fmt::Display for Traversal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.m.m.len() {
            for c in 0..self.m.m[0].len() {
                if self.t.r == r && self.t.c == c {
                    write!(f, "{}", self.t.d)?
                } else {
                    write!(f, "{}", self.m.m[r][c])?
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

fn main() {
    let mut maze = std::io::stdin()
        .lines()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    let commands = maze.pop().unwrap();
    maze.pop();
    let maze = Maze::parse(&maze);
    let commands = Commands::parse(&commands);
    // println!("{}", maze);
    // println!("{}", commands);
    let mut t = Traversal::new(maze, commands);
    // println!("{}", t);
    t.execute();
    println!("Part 1: {}", t.t.password());

    println!(
        "{}, {}",
        std::cmp::max(t.m.m.len(), t.m.m[0].len()) / 4,
        std::cmp::min(t.m.m.len(), t.m.m[0].len()) / 3
    );
}

/*

* If you're not going to wrap, everything works as before.
* If you are going to wrap, your direction may change. But we only care about the direction with respect to the original map.
*

*/
