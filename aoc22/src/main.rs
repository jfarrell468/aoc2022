use std::{collections::HashMap, fmt, io::stdout};

#[derive(Debug, Clone, PartialEq, Eq)]
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
    fn width(&self) -> usize {
        self.m[0].len()
    }
    fn height(&self) -> usize {
        self.m.len()
    }
    fn fmt_with_turtle(&self, f: &mut fmt::Formatter<'_>, t: Option<&Turtle>) -> fmt::Result {
        for r in 0..self.height() {
            for c in 0..self.width() {
                if let Some(t) = t {
                    if t.r == r && t.c == c {
                        write!(f, "{}", t.d);
                    } else {
                        write!(f, "{}", self.m[r][c]);
                    }
                } else {
                    write!(f, "{}", self.m[r][c]);
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_turtle(f, None)
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            c,
        }
    }
    fn offset_tile(&self, m: &Maze, d: usize) -> (usize, usize) {
        match self.d {
            Direction::Right => (self.r, (self.c + d) % m.width()),
            Direction::Down => ((self.r + d) % m.height(), self.c),
            Direction::Left => (self.r, (self.c + m.width() - d) % m.width()),
            Direction::Up => ((self.r + m.height() - d) % m.m.len(), self.c),
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
    fn execute_cube(&mut self, c: &Command, cube: &Cube) {
        match c {
            Command::Forward(dist) => {
                for _ in 0..*dist {
                    let (r, c, d) = self.next_pos_cube(cube);
                    if cube.m.m[r][c] == Contents::Wall {
                        break;
                    } else {
                        self.r = r;
                        self.c = c;
                        self.d = d;
                        // println!("{}", TurtleAndMaze(self, cube.m));
                    }
                }
            }
            Command::RotateLeft => self.d.rotate(c),
            Command::RotateRight => self.d.rotate(c),
        }
    }
    fn next_pos_cube(&self, cube: &Cube) -> (usize, usize, Direction) {
        let face = self.face(cube);
        if face.on_edge_with_direction(self) {
            assert!(self.on_edge(cube));
            let result = face.traverse_edge(self, cube);
            println!("Result of traversal: {:?}", result);
            println!("Face: {}", cube.face((result.0, result.1)).idx);
            assert!(
                self.face(cube).idx != cube.face((result.0, result.1)).idx,
                "Did not change face"
            );
            result
        } else {
            let result = match self.d {
                Direction::Right => (self.r, self.c + 1, self.d),
                Direction::Down => (self.r + 1, self.c, self.d),
                Direction::Left => (self.r, self.c - 1, self.d),
                Direction::Up => (self.r - 1, self.c, self.d),
            };
            assert!(
                self.face(cube).idx == cube.face((result.0, result.1)).idx,
                "Did not stay on same face"
            );
            result
        }
    }
    fn face(&self, cube: &Cube) -> CubeFace {
        cube.face((self.r, self.c))
    }
    fn on_edge(&self, cube: &Cube) -> bool {
        cube.on_edge((self.r, self.c))
    }
    fn password(&self) -> usize {
        1000 * (self.r + 1) + 4 * (self.c + 1) + self.d.to_score()
    }
}

struct TurtleAndMaze<'a, 'b>(&'a Turtle, &'b Maze);
impl fmt::Display for TurtleAndMaze<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.1.fmt_with_turtle(f, Some(self.0))
    }
}

struct Traversal<'a> {
    m: &'a Maze,
    c: Commands,
    t: Turtle,
}
impl Traversal<'_> {
    fn new(m: &Maze, c: Commands) -> Traversal {
        let t = Turtle::new(m);
        Traversal { m, c, t }
    }
    fn execute(&mut self) {
        for c in &self.c.c {
            // println!("Executing {:?}", c);
            self.t.execute(c, &self.m);
            // println!("{}", self);
        }
    }
    fn execute_cube(&mut self) {
        let cube = Cube::from(&self.m);
        // for face in &cube.faces {
        //     println!("{:?}", face);
        // }
        for c in &self.c.c {
            println!("Executing {:?}", c);
            self.t.execute_cube(c, &cube);
        }
    }
}
impl fmt::Display for Traversal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        TurtleAndMaze(&self.t, self.m).fmt(f)
    }
}

struct Cube<'a> {
    m: &'a Maze,
    face_idx: HashMap<(usize, usize), usize>,
    faces: Vec<CubeFace>,
}
impl Cube<'_> {
    fn from(m: &Maze) -> Cube {
        let mut cube = Cube {
            m,
            face_idx: HashMap::new(),
            faces: Vec::new(),
        };
        let mut face = 0;
        for r in 0..cube.faces_per_col() {
            for c in 0..cube.faces_per_row() {
                if cube.m.m[r * cube.face_len()][c * cube.face_len()] != Contents::Invalid {
                    cube.face_idx.insert((r, c), face);
                    cube.faces.push(CubeFace {
                        idx: face,
                        r: r * cube.face_len(),
                        c: c * cube.face_len(),
                        size: cube.face_len(),
                        traversals: HashMap::new(),
                    });
                    print!("{}", face);
                    face += 1;
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("face_len = {}", cube.face_len());
        for face in &cube.faces {
            // println!("{:?}", face);
        }
        for (pos, idx) in &cube.face_idx {
            for t in [
                (pos.0, pos.1 + 1, Direction::Right),
                (pos.0 + 1, pos.1, Direction::Down),
                (pos.0, pos.1 - 1, Direction::Left),
                (pos.0 - 1, pos.1, Direction::Up),
            ] {
                if cube.face_idx.contains_key(&(t.0, t.1)) {
                    // println!("Adding traversal from {} to {} with direction {}", cube.faces[*idx].idx, cube.face_idx[&(t.0, t.1)], t.2);
                    cube.faces[*idx].traversals.insert(
                        t.2,
                        EdgeTraversal {
                            next: cube.face_idx[&(t.0, t.1)],
                            rotations: 0,
                        },
                    );
                }
            }
            for t in [
                (pos.0 - 1, pos.1 - 1, Direction::Up, 3),
                (pos.0 - 1, pos.1 - 1, Direction::Left, 1),
                (pos.0 - 1, pos.1 + 1, Direction::Up, 1),
                (pos.0 - 1, pos.1 + 1, Direction::Right, 3),
                (pos.0 + 1, pos.1 + 1, Direction::Right, 1),
                (pos.0 + 1, pos.1 + 1, Direction::Down, 3),
                (pos.0 + 1, pos.1 - 1, Direction::Down, 1),
                (pos.0 + 1, pos.1 - 1, Direction::Left, 1),
            ] {
                if !cube.faces[*idx].traversals.contains_key(&t.2)
                    && cube.face_idx.contains_key(&(t.0, t.1))
                {
                    // println!("Adding traversal from {} to {} with direction {} and {} rotations", cube.faces[*idx].idx, cube.face_idx[&(t.0, t.1)], t.2, t.3);
                    cube.faces[*idx].traversals.insert(
                        t.2,
                        EdgeTraversal {
                            next: cube.face_idx[&(t.0, t.1)],
                            rotations: t.3,
                        },
                    );
                }
            }
        }
        if cube.face_len() == 50 {
            cube.faces[0].traversals.insert(
                Direction::Up,
                EdgeTraversal {
                    next: 5,
                    rotations: 1,
                },
            );
            cube.faces[5].traversals.insert(
                Direction::Left,
                EdgeTraversal {
                    next: 0,
                    rotations: 3,
                },
            ); // Auto-computed incorrectly.
            cube.faces[3].traversals.insert(
                Direction::Left,
                EdgeTraversal {
                    next: 0,
                    rotations: 2,
                },
            );
            cube.faces[0].traversals.insert(
                Direction::Left,
                EdgeTraversal {
                    next: 3,
                    rotations: 2,
                },
            );
            cube.faces[4].traversals.insert(
                Direction::Right,
                EdgeTraversal {
                    next: 1,
                    rotations: 2,
                },
            );
            cube.faces[5].traversals.insert(
                Direction::Down,
                EdgeTraversal {
                    next: 1,
                    rotations: 0,
                },
            );
            cube.faces[1].traversals.insert(
                Direction::Up,
                EdgeTraversal {
                    next: 5,
                    rotations: 0,
                },
            );
            cube.faces[1].traversals.insert(
                Direction::Right,
                EdgeTraversal {
                    next: 4,
                    rotations: 2,
                },
            );
        }
        cube
    }
    fn face(&self, pos: (usize, usize)) -> CubeFace {
        self.faces[self.face_idx[&(pos.0 / self.face_len(), pos.1 / self.face_len())]].clone()
    }
    fn on_edge(&self, pos: (usize, usize)) -> bool {
        self.faces[self.face_idx[&(pos.0 / self.face_len(), pos.1 / self.face_len())]].on_edge(pos)
    }
    fn face_len(&self) -> usize {
        std::cmp::max(self.m.height(), self.m.width()) / 4
    }
    fn faces_per_row(&self) -> usize {
        self.m.width() / self.face_len()
    }
    fn faces_per_col(&self) -> usize {
        self.m.height() / self.face_len()
    }
}

#[derive(Debug, Clone)]
struct CubeFace {
    idx: usize,
    r: usize,
    c: usize,
    size: usize,
    traversals: HashMap<Direction, EdgeTraversal>,
}
impl CubeFace {
    fn offset(&self, pos: (usize, usize)) -> (usize, usize) {
        assert!(pos.0 >= self.r);
        assert!(pos.1 >= self.c);
        assert!(pos.0 - self.r < self.size);
        assert!(pos.1 - self.c < self.size);
        (pos.0 - self.r, pos.1 - self.c)
    }
    fn on_edge(&self, pos: (usize, usize)) -> bool {
        let (r, c) = self.offset(pos);
        r == 0 || c == 0 || r == self.size - 1 || c == self.size - 1
    }
    fn on_edge_with_direction(&self, t: &Turtle) -> bool {
        let (r, c) = self.offset((t.r, t.c));
        match t.d {
            Direction::Right => c == self.size - 1,
            Direction::Down => r == self.size - 1,
            Direction::Left => c == 0,
            Direction::Up => r == 0,
        }
    }
    fn traverse_edge(&self, t: &Turtle, cube: &Cube) -> (usize, usize, Direction) {
        println!("Traversing edge on face {} with {:?}", self.idx, t);
        assert!(self.on_edge_with_direction(t));
        if !self.traversals.contains_key(&t.d) {
            panic!(
                "Don't know how to move from face {} in direction {}",
                self.idx, t.d
            );
        }
        let traversal = self.traversals[&t.d];
        let next = cube.faces[traversal.next].clone();
        println!(
            "Next face is {} with {} rotations",
            next.idx, traversal.rotations
        );
        // println!("Face: {:?}", next);
        let offset = self.offset((t.r, t.c));
        // println!("Offset on current face: {:?}", offset);
        let result = if traversal.rotations == 0 {
            let next_offset = match t.d {
                Direction::Right => mirror_v(offset, self.size),
                Direction::Down => mirror_h(offset, self.size),
                Direction::Left => mirror_v(offset, self.size),
                Direction::Up => mirror_h(offset, self.size),
            };
            (next.r + next_offset.0, next.c + next_offset.1, t.d)
        } else if traversal.rotations == 1 {
            let (next_offset, next_d) = match t.d {
                Direction::Right => (
                    mirror_h(rotate(offset, self.size), self.size),
                    Direction::Down,
                ),
                Direction::Down => (
                    mirror_v(rotate(offset, self.size), self.size),
                    Direction::Left,
                ),
                Direction::Left => (
                    mirror_h(rotate(offset, self.size), self.size),
                    Direction::Up,
                ),
                Direction::Up => (
                    mirror_v(rotate(offset, self.size), self.size),
                    Direction::Right,
                ),
            };
            // println!("next.r = {}, next_offset.0 = {}, next.c = {}, next_offset.1 = {}", next.r, next_offset.0, next.c, next_offset.1);
            (next.r + next_offset.0, next.c + next_offset.1, next_d)
        } else if traversal.rotations == 2 {
            let (next_offset, next_d) = match t.d {
                Direction::Right => (
                    mirror_v(rotate(rotate(offset, self.size), self.size), self.size),
                    Direction::Left,
                ),
                Direction::Down => (
                    mirror_h(rotate(rotate(offset, self.size), self.size), self.size),
                    Direction::Up,
                ),
                Direction::Left => (
                    mirror_v(rotate(rotate(offset, self.size), self.size), self.size),
                    Direction::Right,
                ),
                Direction::Up => (
                    mirror_h(rotate(rotate(offset, self.size), self.size), self.size),
                    Direction::Down,
                ),
            };
            (next.r + next_offset.0, next.c + next_offset.1, next_d)
        } else if traversal.rotations == 3 {
            let (next_offset, next_d) = match t.d {
                Direction::Right => (
                    mirror_h(
                        rotate(rotate(rotate(offset, self.size), self.size), self.size),
                        self.size,
                    ),
                    Direction::Up,
                ),
                Direction::Down => (
                    mirror_v(
                        rotate(rotate(rotate(offset, self.size), self.size), self.size),
                        self.size,
                    ),
                    Direction::Right,
                ),
                Direction::Left => (
                    mirror_h(
                        rotate(rotate(rotate(offset, self.size), self.size), self.size),
                        self.size,
                    ),
                    Direction::Down,
                ),
                Direction::Up => (
                    mirror_v(
                        rotate(rotate(rotate(offset, self.size), self.size), self.size),
                        self.size,
                    ),
                    Direction::Left,
                ),
            };
            (next.r + next_offset.0, next.c + next_offset.1, next_d)
        } else {
            panic!("Don't know how to rotate {}.", traversal.rotations);
        };
        assert!(
            next.idx == cube.face((result.0, result.1)).idx,
            "Wrong face. Expected {}, got {}",
            next.idx,
            cube.face((result.0, result.1)).idx
        );
        assert!(
            cube.face((result.0, result.1))
                .on_edge((result.0, result.1)),
            "Not on edge of a face"
        );
        result
    }
}

#[derive(Debug, Clone, Copy)]
struct EdgeTraversal {
    next: usize,
    rotations: usize,
}

fn rotate(pos: (usize, usize), size: usize) -> (usize, usize) {
    (pos.1, size - pos.0 - 1)
}
fn mirror_h(pos: (usize, usize), size: usize) -> (usize, usize) {
    (size - pos.0 - 1, pos.1)
}
fn mirror_v(pos: (usize, usize), size: usize) -> (usize, usize) {
    (pos.0, size - pos.1 - 1)
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
    let mut t = Traversal::new(&maze, commands.clone());
    // println!("{}", t);
    t.execute();
    println!("Part 1: {}", t.t.password());

    let mut t = Traversal::new(&maze, commands);
    t.execute_cube();
    println!("Part 2: {}", t.t.password());
}

/*

* If you're not going to wrap, everything works as before.
* If you are going to wrap, your direction may change. But we only care about the direction with respect to the original map.
*

*/
