fn main() {
    let mut most_calories: i32 = 0;
    let mut current_calories: i32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if current_calories > most_calories {
                most_calories = current_calories;
            }
            current_calories = 0;
            continue
        }
        current_calories += line.parse::<i32>().unwrap();
    }
    if current_calories > most_calories {
        most_calories = current_calories;
    }
    println!("{}", most_calories)
}
