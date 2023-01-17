struct Snafu(i64);
impl Snafu {
    fn parse(s: &str) -> Snafu {
        let mut x: i64 = 0;
        let mut place = 1;
        for c in s.chars().rev() {
            x += digit_to_val(c) * place;
            place *= 5;
        }
        Snafu(x)
    }
    fn encode(&self) -> String {
        let mut digits = Vec::new();
        let mut remaining = self.0;
        let mut divisor = 5;
        while remaining > 0 {
            digits.push(remaining % divisor);
            remaining /= 5;
        }
        digits.push(0);
        // println!("{:?}", digits);
        let mut chars = Vec::new();
        for i in 0..(digits.len() - 1) {
            while digits[i] >= 5 {
                digits[i] -= 5;
                digits[i + 1] += 1;
            }
            match digits[i] {
                0 => {
                    chars.push("0");
                }
                1 => {
                    chars.push("1");
                }
                2 => {
                    chars.push("2");
                }
                3 => {
                    chars.push("=");
                    digits[i + 1] += 1;
                }
                4 => {
                    chars.push("-");
                    digits[i + 1] += 1;
                }
                _ => panic!(),
            }
        }
        match digits[digits.len() - 1] {
            0 => {}
            1 => {
                chars.push("1");
            }
            2 => {
                chars.push("2");
            }
            _ => panic!(),
        }
        chars.reverse();
        chars.join("")
    }
}

fn digit_to_val(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!(),
    }
}

fn main() {
    let mut total = Snafu(0);
    for line in std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
    {
        let x = Snafu::parse(&line);
        let rt = x.encode();
        // println!("{} = {} = {}", line, x.0, rt);
        assert!(line == rt);
        total.0 += x.0;
    }
    println!("{}", total.encode());
}
