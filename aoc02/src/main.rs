#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
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

impl Outcome {
    fn parse(c: char) -> Result<Outcome, ParseError> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(ParseError),
        }
    }
    fn my_play(self, their_play: Play) -> Play {
        match self {
            Outcome::Loss => match their_play {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            Outcome::Draw => their_play,
            Outcome::Win => match their_play {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
        }
    }
}

fn main() {
    let mut part1_score: i32 = 0;
    let mut part2_score: i32 = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        let their_play = Play::parse(chars.next().unwrap()).unwrap();
        chars.next();
        let third_char = chars.next().unwrap();

        let my_play = Play::parse(third_char).unwrap();
        part1_score += my_play.score_versus(their_play);

        let desired_outcome = Outcome::parse(third_char).unwrap();
        let my_play = desired_outcome.my_play(their_play);
        part2_score += my_play as i32 + desired_outcome as i32;
    }
    println!("Part 1: {}", part1_score);
    println!("Part 2: {}", part2_score);
}
