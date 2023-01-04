#[derive(Debug)]
struct ParseError;

fn main() {
    let mut part1_count = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut tokens = line.split(&[',', '-']);
        let r1 = std::ops::Range { start: tokens.next().unwrap().parse::<i32>().unwrap(), end: tokens.next().unwrap().parse::<i32>().unwrap() + 1 };
        let r2 = std::ops::Range { start: tokens.next().unwrap().parse::<i32>().unwrap(), end: tokens.next().unwrap().parse::<i32>().unwrap() + 1 };
        if r1.contains(&r2.start) && r1.contains(&(r2.end - 1)) || r2.contains(&r1.start) && r2.contains(&(r1.end - 1)) {
            part1_count += 1;
        }
    }
    println!("Part 1: {}", part1_count);
}
