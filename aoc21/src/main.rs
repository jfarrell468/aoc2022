use std::collections::{HashMap, LinkedList};

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct WaitingMonkey {
    name: String,
    m1: String,
    op: Operation,
    m2: String,
}
impl WaitingMonkey {
    fn compute(&self, hm: &HashMap<String, i64>) -> Option<i64> {
        match (hm.get(&self.m1), hm.get(&self.m2)) {
            (Some(m1), Some(m2)) => Some(match self.op {
                Operation::Add => m1 + m2,
                Operation::Sub => m1 - m2,
                Operation::Mul => m1 * m2,
                Operation::Div => m1 / m2,
            }),
            _ => None,
        }
    }
}

fn main() {
    let mut happy_monkeys = HashMap::new();
    let mut waiting_monkeys = LinkedList::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match line.split(&[':', ' ']).collect::<Vec<_>>().as_slice() {
            [m, _, lit] => {
                happy_monkeys.insert(m.to_string(), lit.parse::<i64>().unwrap());
            }
            [m, _, m1, op, m2] => waiting_monkeys.push_back(WaitingMonkey {
                name: m.to_string(),
                m1: m1.to_string(),
                op: match op.chars().next().unwrap() {
                    '+' => Operation::Add,
                    '-' => Operation::Sub,
                    '*' => Operation::Mul,
                    '/' => Operation::Div,
                    _ => panic!(),
                },
                m2: m2.to_string(),
            }),
            _ => panic!(),
        }
    }
    // println!("{:?}", happy_monkeys);
    // println!("{:?}", waiting_monkeys);
    while !waiting_monkeys.is_empty() {
        let m = waiting_monkeys.pop_front().unwrap();
        match m.compute(&happy_monkeys) {
            Some(x) => {
                happy_monkeys.insert(m.name.clone(), x);
            }
            None => {
                waiting_monkeys.push_back(m);
            }
        }
    }
    println!("Part 1: {}", happy_monkeys["root"]);
}
