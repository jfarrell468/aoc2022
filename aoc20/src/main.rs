fn print(f: &Vec<(i64, usize)>) {
    println!(
        "{}",
        f.iter()
            .map(|e| format!("{}", e.0))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn decrypt(f: &Vec<i64>, key: i64, cycles: i64) -> Vec<i64> {
    // println!("Decrypt with key {} and {} cycles", key, cycles);
    let mut ff = Vec::new();
    for i in 0..f.len() {
        ff.push((f[i] * key, i));
    }
    // print(&ff);
    for cycle in 0..cycles {
        // println!("\nCycle {}", cycle);
        for i in 0..f.len() {
            for j in 0..ff.len() {
                if i == ff[j].1 {
                    shift(&mut ff, j);
                    break;
                }
            }
        }
        // print(&ff);
    }
    return ff.iter().map(|e| e.0).collect::<Vec<_>>();
}

fn shift(f: &mut Vec<(i64, usize)>, i: usize) {
    assert!(i < f.len());
    let m = f.len() - 1;
    let move_by = f[i].0 % m as i64;
    // println!("Move {} by {}", f[i].0, move_by);
    if move_by == 0 {
        return;
    }
    let mut dest_idx = ((i as i64 + f[i].0) % (m as i64) + (m as i64)) as usize % m;
    // if dest_idx == 0 {
    //     dest_idx = m;
    // }
    // println!("Move {} from {} to {}", f[i].0, i, dest_idx);
    if dest_idx > i {
        for j in i..dest_idx {
            f.swap(j, j + 1);
        }
    } else if dest_idx < i {
        for j in (dest_idx..i).rev() {
            f.swap(j, j + 1);
        }
    }
    // print(&f);
}

fn get_coordinates(f: &Vec<i64>) -> i64 {
    for i in 0..f.len() {
        if f[i] == 0 {
            let f1000 = f[(i + 1000) % f.len()];
            let f2000 = f[(i + 2000) % f.len()];
            let f3000 = f[(i + 3000) % f.len()];
            return f1000 + f2000 + f3000;
        }
    }
    panic!();
}

fn main() {
    let mut f = Vec::new();
    for line in std::io::stdin().lines() {
        f.push(line.unwrap().parse::<i64>().unwrap());
    }

    println!("Part 1: {}", get_coordinates(&decrypt(&f, 1, 1)));
    println!("Part 2: {}", get_coordinates(&decrypt(&f, 811589153, 10)));
}
