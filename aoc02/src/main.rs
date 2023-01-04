#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug)]
struct ParseError;

impl Play {
    fn parse(c: char) -> Result<Play, ParseError> {
        match c {
            'A' | 'X' => Ok(Play::Rock),
            'B' | 'Y' => Ok(Play::Paper),
            'C' | 'Z' => Ok(Play::Scissors),
            _ => Err(ParseError),
        }
    }
    fn outcome_versus(self, opponent: Play) -> Outcome {
        match self {
            Play::Rock => match opponent {
                Play::Rock => Outcome::Draw,
                Play::Paper => Outcome::Loss,
                Play::Scissors => Outcome::Win,
            },
            Play::Paper => match opponent {
                Play::Rock => Outcome::Win,
                Play::Paper => Outcome::Draw,
                Play::Scissors => Outcome::Loss,
            },
            Play::Scissors => match opponent {
                Play::Rock => Outcome::Loss,
                Play::Paper => Outcome::Win,
                Play::Scissors => Outcome::Draw,
            },
        }
    }
    fn score_versus(self, opponent: Play) -> i32 {
        self as i32 + self.outcome_versus(opponent) as i32
    }
}

fn main() {
    let mut score: i32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let their_play = Play::parse(chars.next().unwrap()).unwrap();
        chars.next();
        let my_play = Play::parse(chars.next().unwrap()).unwrap();
        score += my_play.score_versus(their_play);
    }
    println!("Part 1: {}", score);
}
