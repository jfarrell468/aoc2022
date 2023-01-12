fn main() {
    let mut drop_vec = Vec::new();
    let mut min = (-1, -1, -1);
    let mut max = (-1, -1, -1);
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(",").collect();
        let x = tokens[0].parse::<i32>().unwrap();
        let y = tokens[1].parse::<i32>().unwrap();
        let z = tokens[2].parse::<i32>().unwrap();
        println!("{}, {}, {}", x, y, z);
        if min.0 == -1 {
            min = (x, y, z);
            max = (x, y, z);
        } else {
            min = (
                std::cmp::min(min.0, x),
                std::cmp::min(min.1, y),
                std::cmp::min(min.2, z),
            );
            max = (
                std::cmp::max(max.0, x),
                std::cmp::max(max.1, y),
                std::cmp::max(max.2, z),
            );
        }
        // println!("min = {:?}, max = {:?}", min, max);
        drop_vec.push((x, y, z));
    }
    println!("final min = {:?}, max = {:?}", min, max);
    let mut drops = vec![
        vec![
            vec![false; (max.2 - min.2 + 3).try_into().unwrap()];
            (max.1 - min.1 + 3).try_into().unwrap()
        ];
        (max.0 - min.0 + 3).try_into().unwrap()
    ];
    for drop in drop_vec {
        drops[(drop.0 - min.0 + 1) as usize][(drop.1 - min.1 + 1) as usize]
            [(drop.2 - min.2 + 1) as usize] = true;
    }
    let mut surface_area = 0;
    for x in 1..(max.0 - min.0 + 2) {
        for y in 1..(max.1 - min.1 + 2) {
            for z in 1..(max.2 - min.2 + 2) {
                for delta in [
                    (-1, 0, 0),
                    (1, 0, 0),
                    (0, -1, 0),
                    (0, 1, 0),
                    (0, 0, -1),
                    (0, 0, 1),
                ] {
                    if drops[x as usize][y as usize][z as usize]
                        && !drops[(x + delta.0) as usize][(y + delta.1) as usize]
                            [(z + delta.2) as usize]
                    {
                        surface_area += 1;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", surface_area);

    let mut is_exterior = vec![
        vec![
            vec![false; (max.2 - min.2 + 3).try_into().unwrap()];
            (max.1 - min.1 + 3).try_into().unwrap()
        ];
        (max.0 - min.0 + 3).try_into().unwrap()
    ];
    is_exterior[0][0][0] = true;
    let mut exterior_check: Vec<(i32, i32, i32)> = Vec::new();
    exterior_check.push((0, 0, 0));
    while !exterior_check.is_empty() {
        let check = exterior_check.pop().unwrap();
        for delta in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let x = check.0 + delta.0;
            let y = check.1 + delta.1;
            let z = check.2 + delta.2;
            if x >= 0
                && x < drops.len() as i32
                && y >= 0
                && y < drops[0].len() as i32
                && z >= 0
                && z < drops[0][0].len() as i32
            {
                if !is_exterior[x as usize][y as usize][z as usize]
                    && !drops[x as usize][y as usize][z as usize]
                {
                    is_exterior[x as usize][y as usize][z as usize] = true;
                    exterior_check.push((x, y, z));
                }
            }
        }
    }
    let mut surface_area = 0;
    for x in 1..(max.0 - min.0 + 2) {
        for y in 1..(max.1 - min.1 + 2) {
            for z in 1..(max.2 - min.2 + 2) {
                for delta in [
                    (-1, 0, 0),
                    (1, 0, 0),
                    (0, -1, 0),
                    (0, 1, 0),
                    (0, 0, -1),
                    (0, 0, 1),
                ] {
                    if drops[x as usize][y as usize][z as usize]
                        && !drops[(x + delta.0) as usize][(y + delta.1) as usize]
                            [(z + delta.2) as usize]
                        && is_exterior[(x + delta.0) as usize][(y + delta.1) as usize]
                            [(z + delta.2) as usize]
                    {
                        surface_area += 1;
                    }
                }
            }
        }
    }
    println!("Part 2: {}", surface_area);
}
