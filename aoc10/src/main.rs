use std::collections::HashSet;

enum Op {
    Noop,
    Addx(i32),
}

fn main() {
    let mut program = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        if tokens[0] == "noop" {
            program.push(Op::Noop);
        } else if tokens[0] == "addx" {
            program.push(Op::Noop);
            program.push(Op::Addx(tokens[1].parse::<i32>().unwrap()));
        } else {
            panic!();
        }
    }
    let interesting_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut ss = 0;
    let mut pc = 1;
    let mut x = 1;
    for op in &program {
        if interesting_cycles.contains(&pc) {
            println!("Cycle {}, x = {}, product = {}", pc, x, pc * x);
            ss += pc * x;
        }
        if let Op::Addx(operand) = op {
            x += operand;
        }
        pc += 1;
    }
    println!("Part 1: {}", ss);

    let w = 40;
    let mut crt = Vec::new();
    let mut crt_line = String::new();
    let mut pc = 1;
    let mut x = 1;
    for op in &program {
        let pixel = (pc - 1) % w;
        crt_line.push(if pixel == x || pixel == x - 1 || pixel == x + 1 {
            '#'
        } else {
            '.'
        });
        // println!("{}", crt_line);
        if crt_line.len() == w as usize {
            crt.push(crt_line);
            crt_line = String::from("");
            if crt.len() == 6 {
                println!("{}\n", crt.join("\n"));
            }
        }
        if let Op::Addx(operand) = op {
            x += operand;
        }
        pc += 1;
    }
    println!("{}\n", crt.join("\n"));
}
