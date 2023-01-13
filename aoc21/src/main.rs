use std::collections::{HashMap, LinkedList};
use std::fmt;

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        })
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Unknown(String),
    Known(i64),
}
impl Operand {
    fn simplify(&mut self, hm: &HashMap<String, i64>) -> Option<i64> {
        match self {
            Operand::Unknown(name) => match hm.get(name) {
                Some(val) => {
                    *self = Operand::Known(*val);
                    Some(*val)
                }
                None => None,
            },
            Operand::Known(val) => Some(*val),
        }
    }
}
impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Unknown(x) => f.write_str(x),
            Operand::Known(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    name: String,
    v1: Operand,
    op: Operator,
    v2: Operand,
}
impl Operation {
    fn simplify(&mut self, hm: &mut HashMap<String, i64>) -> Option<i64> {
        match (self.v1.simplify(hm), self.v2.simplify(hm)) {
            (Some(m1), Some(m2)) => {
                let val = match self.op {
                    Operator::Add => m1 + m2,
                    Operator::Sub => m1 - m2,
                    Operator::Mul => m1 * m2,
                    Operator::Div => m1 / m2,
                };
                hm.insert(self.name.clone(), val);
                Some(val)
            }
            _ => None,
        }
    }
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} {} {}", self.name, self.v1, self.op, self.v2)
    }
}

struct Goal {
    v1: Operand,
    v2: Operand,
}
impl Goal {
    fn simplify(&mut self, hm: &HashMap<String, i64>, ops: &HashMap<String, Operation>) {
        self.v1.simplify(hm);
        self.v2.simplify(hm);
        let (mut goal, mut unknown) = match (&self.v1, &self.v2) {
            (Operand::Unknown(_), Operand::Unknown(_)) => panic!(),
            (Operand::Unknown(u), Operand::Known(k)) => (*k, u.clone()),
            (Operand::Known(k), Operand::Unknown(u)) => (*k, u.clone()),
            (Operand::Known(_), Operand::Known(_)) => panic!(),
        };
        let mut op = ops[&unknown].clone();
        while !hm.contains_key(&op.name) {
            println!("{} == {}", op, goal);
            match (&op.v1, &op.v2) {
                (Operand::Unknown(_), Operand::Unknown(_)) => panic!(),
                (Operand::Unknown(u), Operand::Known(k)) => {
                    match op.op {
                        Operator::Add => {
                            goal -= *k;
                        }
                        Operator::Sub => {
                            goal += *k;
                        }
                        Operator::Mul => {
                            goal /= *k;
                        }
                        Operator::Div => {
                            goal *= *k;
                        }
                    }
                    if !ops.contains_key(u) {
                        println!("Key not found: {}. Goal is {}", op.name, goal);
                        return;
                    }
                    op = ops[u].clone();
                }
                (Operand::Known(k), Operand::Unknown(u)) => {
                    match op.op {
                        Operator::Add => {
                            goal -= *k;
                        }
                        Operator::Sub => {
                            goal = *k - goal;
                        }
                        Operator::Mul => {
                            goal /= *k;
                        }
                        Operator::Div => panic!(),
                    }
                    if !ops.contains_key(u) {
                        println!("Key not found: {}. Goal is {}", op.name, goal);
                        return;
                    }
                    op = ops[u].clone();
                }
                (Operand::Known(_), Operand::Known(_)) => panic!(),
            }
        }
    }
}

fn main() {
    let mut happy_monkeys = HashMap::new();
    let mut operations = LinkedList::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        match line.split(&[':', ' ']).collect::<Vec<_>>().as_slice() {
            [m, _, lit] => {
                happy_monkeys.insert(m.to_string(), lit.parse::<i64>().unwrap());
            }
            [m, _, m1, op, m2] => operations.push_back(Operation {
                name: m.to_string(),
                v1: Operand::Unknown(m1.to_string()),
                op: match op.chars().next().unwrap() {
                    '+' => Operator::Add,
                    '-' => Operator::Sub,
                    '*' => Operator::Mul,
                    '/' => Operator::Div,
                    _ => panic!(),
                },
                v2: Operand::Unknown(m2.to_string()),
            }),
            _ => panic!(),
        }
    }

    let mut hm_part1 = happy_monkeys.clone();
    let mut ops_part1 = operations.clone();
    // println!("{:?}", happy_monkeys);
    // println!("{:?}", waiting_monkeys);
    while !ops_part1.is_empty() {
        let mut o = ops_part1.pop_front().unwrap();
        if o.simplify(&mut hm_part1).is_none() {
            ops_part1.push_back(o);
        }
    }
    println!("Part 1: {}", hm_part1["root"]);
    drop(ops_part1);
    drop(hm_part1);

    happy_monkeys.remove("humn");
    let mut ops = HashMap::new();
    let mut goals = LinkedList::new();
    for op in &operations {
        if op.name == "root" {
            goals.push_back(Goal {
                v1: op.v1.clone(),
                v2: op.v2.clone(),
            });
        } else {
            ops.insert(op.name.clone(), op.clone());
        }
    }
    drop(operations);
    loop {
        let mut num_known = happy_monkeys.len();
        ops.retain(|k, v| v.simplify(&mut happy_monkeys).is_none());
        if num_known == happy_monkeys.len() {
            break;
        }
        num_known = happy_monkeys.len();
    }
    for goal in &mut goals {
        goal.simplify(&happy_monkeys, &ops);
    }
}
