#[derive(Debug, Clone)]
enum Operand {
    Old,
    Literal(i64),
}
impl Operand {
    fn parse(s: &str) -> Operand {
        if s == "old" {
            Operand::Old
        } else {
            Operand::Literal(s.parse::<i64>().unwrap())
        }
    }
    fn value(&self, old: i64) -> i64 {
        match *self {
            Operand::Old => old,
            Operand::Literal(val) => val,
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}
impl Operator {
    fn parse(s: &str) -> Operator {
        if s == "*" {
            Operator::Multiply
        } else {
            Operator::Add
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    op1: Operand,
    operator: Operator,
    op2: Operand,
}
impl Operation {
    fn apply(&self, worry: i64) -> i64 {
        let v1 = self.op1.value(worry);
        let v2 = self.op2.value(worry);
        match self.operator {
            Operator::Add => v1 + v2,
            Operator::Multiply => v1 * v2,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspections: i64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: Operation {
                op1: Operand::Old,
                operator: Operator::Add,
                op2: Operand::Old,
            },
            test: 0,
            if_true: 0,
            if_false: 0,
            inspections: 0,
        }
    }
    fn inspect(&mut self, divisor: Option<i64>, modulo: Option<i64>) -> Vec<(i64, usize)> {
        let mut throws: Vec<(i64, usize)> = Vec::new();
        for item in &mut self.items {
            // println!("  Monkey inspects an item with a worry level of {}.", item);
            self.inspections += 1;
            let starting_worry = match modulo {
                Some(m) => *item % m,
                None => *item,
            };
            let worry = match modulo {
                Some(m) => self.operation.apply(starting_worry) / divisor.unwrap_or(1) % m,
                None => self.operation.apply(starting_worry) / divisor.unwrap_or(1),
            };
            if worry < 0 {
                panic!("Apply {:?} to {} gives {}", self.operation, item, worry);
            }
            // println!("    New worry is {}.", worry);
            if worry % self.test == 0 {
                // println!("    Current worry level is divisible by {}.", self.test);
                throws.push((worry, self.if_true));
                // println!("    Item with worry level {} is thrown to monkey {}.", worry, self.if_true);
            } else {
                // println!("    Current worry level is not divisible by {}.", self.test);
                throws.push((worry, self.if_false));
                // println!("    Item with worry level {} is thrown to monkey {}.", worry, self.if_false);
            }
        }
        self.items.clear();
        throws
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut modulo = 1;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        if tokens[0] == "Monkey" {
            monkeys.push(Monkey::new());
        } else if tokens[0] == "Starting" {
            for i in 2..tokens.len() {
                let item;
                if tokens[i].ends_with(",") {
                    item = tokens[i].split_once(",").unwrap().0.parse::<i64>().unwrap();
                } else {
                    item = tokens[i].parse::<i64>().unwrap();
                }
                monkeys.last_mut().unwrap().items.push(item)
            }
        } else if tokens[0] == "Operation:" {
            monkeys.last_mut().unwrap().operation.op1 = Operand::parse(tokens[3]);
            monkeys.last_mut().unwrap().operation.operator = Operator::parse(tokens[4]);
            monkeys.last_mut().unwrap().operation.op2 = Operand::parse(tokens[5]);
        } else if tokens[0] == "Test:" {
            monkeys.last_mut().unwrap().test = tokens[3].parse::<i64>().unwrap();
            modulo *= monkeys.last_mut().unwrap().test;
        } else if tokens[1] == "true:" {
            monkeys.last_mut().unwrap().if_true = tokens[5].parse::<usize>().unwrap();
        } else if tokens[1] == "false:" {
            monkeys.last_mut().unwrap().if_false = tokens[5].parse::<usize>().unwrap();
        }
    }
    let mut monkeys_part2 = monkeys.clone();
    // for monkey in &mut monkeys {
    //     println!("{:?}", monkey);
    // }
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // println!("Monkey {}:", i);
            for throws in monkeys[i].inspect(Some(3), None) {
                monkeys[throws.1].items.push(throws.0)
            }
        }
    }
    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<i64>>();
    inspections.sort();
    inspections.reverse();
    println!("Part 1: {}", inspections[0] * inspections[1]);

    for _ in 0..10000 {
        for i in 0..monkeys_part2.len() {
            // println!("Monkey {}:", i);
            for throws in monkeys_part2[i].inspect(None, Some(modulo)) {
                monkeys_part2[throws.1].items.push(throws.0)
            }
        }
    }
    let mut inspections = monkeys_part2
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<i64>>();
    inspections.sort();
    inspections.reverse();
    println!("Part 2: {}", inspections[0] * inspections[1]);
}
