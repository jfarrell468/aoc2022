use serde_json::Value;
use std::cmp::Ordering;

fn compare(a: &Value, b: &Value) -> std::cmp::Ordering {
    match a {
        Value::Number(aa) => match b {
            Value::Number(bb) => aa.as_i64().unwrap().cmp(&bb.as_i64().unwrap()),
            Value::Array(_) => compare(&Value::Array(vec![Value::Number(aa.clone())]), b),
            _ => panic!(),
        },
        Value::Array(aa) => match b {
            Value::Number(bb) => compare(a, &Value::Array(vec![Value::Number(bb.clone())])),
            Value::Array(bb) => {
                for i in 0..std::cmp::min(aa.len(), bb.len()) {
                    let ordering = compare(&aa[i], &bb[i]);
                    if let Ordering::Equal = ordering {
                        continue;
                    }
                    return ordering;
                }
                aa.len().cmp(&bb.len())
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn main() {
    let d1: Value = serde_json::from_str("[[2]]").unwrap();
    let d2: Value = serde_json::from_str("[[6]]").unwrap();
    let mut packets: Vec<Value> = vec![d1.clone(), d2.clone()];
    let mut lines = std::io::stdin().lines();
    let mut idx = 1;
    let mut idx_sum = 0;
    while let Some(line) = lines.next() {
        let a: Value = serde_json::from_str(line.unwrap().as_str()).unwrap();
        let b: Value = serde_json::from_str(lines.next().unwrap().unwrap().as_str()).unwrap();
        lines.next();
        println!("{}", a.to_string());
        println!("{}", b.to_string());
        println!(
            "a {} b\n",
            match compare(&a, &b) {
                Ordering::Less => "<",
                Ordering::Equal => "==",
                Ordering::Greater => ">",
            }
        );
        if let Ordering::Less = compare(&a, &b) {
            idx_sum += idx;
        }
        idx += 1;

        packets.push(a);
        packets.push(b);
    }
    println!("Part 1: {}", idx_sum);

    packets.sort_by(|a, b| compare(a, b));
    println!(
        "Part 2: {}",
        (packets.binary_search_by(|a| compare(a, &d1)).unwrap() + 1)
            * (packets.binary_search_by(|a| compare(a, &d2)).unwrap() + 1)
    );
}
