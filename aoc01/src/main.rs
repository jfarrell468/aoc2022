fn main() {
    let mut calories = Vec::new();
    let mut current_calories: i32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
            continue
        }
        current_calories += line.parse::<i32>().unwrap();
    }
    calories.push(current_calories);
    calories.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", calories[0]);
    println!("Part 2: {}", calories[0] + calories[1] + calories[2])
}
