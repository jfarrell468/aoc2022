fn main() {
    let mut piles : Vec<Vec<char>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.contains("[") {
            for c in line.char_indices() {
                if c.1.is_ascii_alphabetic() {
                    let pile = c.0 / 4;
                    while piles.len() <= pile {
                        piles.push(Vec::new());
                    }
                    piles[pile].push(c.1);
                }
            }
        } else if line.starts_with("move") {
            let mut tokens = line.split(" ");
            tokens.next();
            let count = tokens.next().unwrap().parse::<usize>().unwrap();
            tokens.next();
            let from = tokens.next().unwrap().parse::<usize>().unwrap();
            tokens.next();
            let to = tokens.next().unwrap().parse::<usize>().unwrap();
            // println!("move {} from {} to {}", count, from, to);
            for _ in 0..count {
                let c = piles[from-1].pop().unwrap();
                piles[to-1].push(c);
            }
        } else if line.is_empty() {
            for pile in piles.iter_mut() {
                pile.reverse();
                // println!("{:?}", pile);
            }
        }
    }
    for pile in piles.iter_mut() {
        print!("{}", pile.pop().unwrap());
    }
    println!("")
}