fn print(f: &Vec<(i32, bool)>) {
    println!(
        "{:?}",
        f.iter()
            .map(|e| format!("{}{}", e.0, if e.1 { "*" } else { "" }))
            .collect::<Vec<_>>()
    );
}

fn wrap(i: i32, delta: i32) {}

fn main() {
    let mut f = Vec::new();
    for line in std::io::stdin().lines() {
        f.push((line.unwrap().parse::<i32>().unwrap(), false));
    }
    // print(&f);
    // println!("-7%7 = {}", -7 % 7);
    // println!("-10%7 = {}", -10 % 7);
    // println!("-3%7 = {}", -3 % 7);
    // println!("(-3%7)+7 = {}", -3 % 7 + 7);
    let mut i: i32 = 1;
    while i <= f.len() as i32 {
        if !f[i as usize - 1].1 {
            let m = f.len() as i32 - 1;
            let mut dest_idx = ((i - 1 + f[i as usize - 1].0) % m + m) % m;
            if dest_idx == 0 {
                dest_idx = m;
            }
            // println!("\nMove {} from {} to {}", f[i as usize - 1].0, i-1, dest_idx);
            f[i as usize - 1].1 = true;
            if dest_idx > i - 1 {
                for j in (i - 1)..dest_idx {
                    f.swap(j as usize, j as usize + 1);
                }
                i -= 1;
            } else if dest_idx < i - 1 {
                for j in (dest_idx..(i - 1)).rev() {
                    f.swap(j as usize, j as usize + 1);
                }
            }
            // print(&f);
        }
        i += 1;
    }
    for i in 0..f.len() {
        if f[i].0 == 0 {
            println!("Found 0 at index {}", i);
            let f1000 = f[(i + 1000) % f.len()].0;
            let f2000 = f[(i + 2000) % f.len()].0;
            let f3000 = f[(i + 3000) % f.len()].0;
            println!("1000th: {}", f1000);
            println!("2000th: {}", f2000);
            println!("3000th: {}", f3000);
            println!("Part 1: {}", f1000 + f2000 + f3000);
            break;
        }
    }
}
