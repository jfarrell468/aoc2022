use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn priority(&self) -> i32 {
        if self.0.is_ascii_lowercase() {
            1 + self.0 as i32 - 'a' as i32
        } else if self.0.is_ascii_uppercase() {
            27 + self.0 as i32 - 'A' as i32
        } else {
            panic!("Invalid priority")
        }
    }
}

struct Rucksack {
    c1: HashSet<Item>,
    c2: HashSet<Item>,
}

impl Rucksack {
    fn parse(contents: &str) -> Rucksack {
        let mut c1 = HashSet::new();
        let mut c2 = HashSet::new();
        let mut len = 0;
        for c in contents.chars() {
            len += 1
        }
        let mut idx = 0;
        for c in contents.chars() {
            if idx >= len / 2 {
                c2.insert(Item(c));
            } else {
                c1.insert(Item(c));
            }
            idx += 1
        }
        Rucksack { c1, c2 }
    }
}

fn main() {
    let mut priority = 0;
    for line in std::io::stdin().lines() {
        let rs = Rucksack::parse(line.unwrap().as_str());
        let mut intersect = rs.c1.intersection(&rs.c2);
        priority += intersect.next().unwrap().priority();
    }
    println!("Part 1: {}", priority)
}
