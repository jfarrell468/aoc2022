use std::collections::HashSet;

fn find_distinct(chars: &Vec<char>, count: usize) -> usize {
    for i in count..chars.len() {
        let last: HashSet<&char> = HashSet::from_iter(chars.get(i - count + 1..=i).unwrap());
        if last.len() == count {
            return i + 1;
        }
    }
    return 0;
}

fn main() {
    let chars: Vec<char> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();
    println!("Part 1: {}", find_distinct(&chars, 4));
    println!("Part 2: {}", find_distinct(&chars, 14));
}
