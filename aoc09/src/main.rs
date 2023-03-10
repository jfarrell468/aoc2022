use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[rustfmt::skip]
const moves: [[(i32, i32); 5]; 5] = [
    [( 1, 1), ( 1, 1), ( 1, 0), ( 1,-1), ( 1,-1)],
    [( 1, 1), ( 0, 0), ( 0, 0), ( 0, 0), ( 1,-1)],
    [( 0, 1), ( 0, 0), ( 0, 0), ( 0, 0), ( 0,-1)],
    [(-1, 1), ( 0, 0), ( 0, 0), ( 0, 0), (-1,-1)],
    [(-1, 1), (-1, 1), (-1, 0), (-1,-1), (-1,-1)],
];

fn move_tail(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    println!("move_tail({:?}, {:?})", h, t);
    let delta = moves[(h.0 - t.0 + 2) as usize][(h.1 - t.1 + 2) as usize];
    (t.0 - delta.0, t.1 - delta.1)
}

fn main() {
    let mut directions = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let d = match tokens[0].chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        };
        directions.push((d, tokens[1].parse::<usize>().unwrap()));
    }

    let mut seen = HashSet::new();
    let mut h = (0, 0);
    let mut t = (0, 0);
    // println!("Head: {:?}, Tail: {:?}", h, t);
    seen.insert(t);
    for (d, count) in &directions {
        for _ in 0..*count {
            h = match d {
                Direction::Up => (h.0 - 1, h.1),
                Direction::Down => (h.0 + 1, h.1),
                Direction::Left => (h.0, h.1 - 1),
                Direction::Right => (h.0, h.1 + 1),
            };
            t = move_tail(h, t);
            // println!("Moved {:?}. Head: {:?}, Tail: {:?}", d, h, t);
            seen.insert(t);
        }
    }
    println!("Part 1: {}", seen.len());

    let mut seen = HashSet::new();
    let mut rope = [(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0)];
    println!("Rope: {:?}", rope);
    seen.insert(t);
    for (d, count) in &directions {
        for _ in 0..*count {
            println!("Move {:?}", d);
            rope[0] = match d {
                Direction::Up => (rope[0].0 - 1, rope[0].1),
                Direction::Down => (rope[0].0 + 1, rope[0].1),
                Direction::Left => (rope[0].0, rope[0].1 - 1),
                Direction::Right => (rope[0].0, rope[0].1 + 1),
            };
            for i in 1..rope.len() {
                rope[i] = move_tail(rope[i-1], rope[i]);
            }
            println!("  Rope: {:?}", rope);
            seen.insert(rope[9]);
        }
    }
    println!("Part 1: {}", seen.len());
}
