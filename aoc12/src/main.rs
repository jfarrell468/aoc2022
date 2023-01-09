fn steps_to_goal(
    h: &Vec<Vec<usize>>,
    s: (usize, usize),
    e: (usize, usize),
    max_steps: usize,
) -> usize {
    let nr = h.len();
    let nc = h[0].len();
    let mut reachable = vec![vec![false; nc]; nr];
    let mut t = 0;
    reachable[s.0][s.1] = true;
    while !reachable[e.0][e.1] {
        let mut rn = vec![vec![false; nc]; nr];
        for r in 0..nr {
            for c in 0..nc {
                let mut last_pos = Vec::new();
                if r > 0 {
                    last_pos.push((r - 1, c));
                }
                if r < h.len() - 1 {
                    last_pos.push((r + 1, c));
                }
                if c > 0 {
                    last_pos.push((r, c - 1));
                }
                if c < h[0].len() - 1 {
                    last_pos.push((r, c + 1));
                }
                for p in last_pos {
                    if reachable[p.0][p.1] && h[r][c] <= h[p.0][p.1] + 1 {
                        rn[r][c] = true;
                    }
                }
            }
        }
        reachable = rn;
        // for r in &reachable {
        //     for c in r {
        //         print!("{}", if *c { "X" } else { "." });
        //     }
        //     println!("");
        // }
        // println!("");
        t += 1;
        if t > max_steps {
            break;
        }
    }
    t
}

fn main() {
    let mut h = Vec::new();
    let mut s = (0, 0);
    let mut e = (0, 0);
    for line in std::io::stdin().lines() {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            if c == 'S' {
                s = (h.len(), row.len());
                row.push(0 as usize);
            } else if c == 'E' {
                e = (h.len(), row.len());
                row.push(25 as usize);
            } else {
                row.push(c as usize - 'a' as usize);
            }
        }
        h.push(row);
    }
    let nr = h.len();
    let nc = h[0].len();
    let mut min_steps = steps_to_goal(&h, s, e, nr * nc);
    println!("Part 1: {}", min_steps);

    for r in 0..nr {
        for c in 0..nc {
            if h[r][c] == 0 {
                let steps = steps_to_goal(&h, (r, c), e, min_steps);
                println!("{} steps from {:?} to {:?}", steps, (r, c), e);
                if steps < min_steps {
                    min_steps = steps;
                }
            }
        }
    }
    println!("Part 2: {}", min_steps)
}
