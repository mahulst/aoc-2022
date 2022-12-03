use std::str::FromStr;

#[derive(Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            _ => Err("Can't parse play".to_string()),
        }
    }
}

const WIN: usize = 6;
const TIE: usize = 3;
const LOSE: usize = 0;

pub fn run_part_1(input: &str) -> String {
    let a: usize = input.lines().map(|l| {
        let vec: Vec<Play> = l.trim().split_whitespace().map(|p| p.parse::<Play>().unwrap()).collect();
        let left = vec.first().unwrap();
        let right = vec.last().unwrap();
        match (left, right) {
            (Play::Paper, Play::Rock) => 1 + LOSE,
            (Play::Paper, Play::Paper) => 2 + TIE,
            (Play::Paper, Play::Scissors) => 3 + WIN,
            (Play::Rock, Play::Rock) => 1 + TIE,
            (Play::Rock, Play::Paper) => 2 + WIN,
            (Play::Rock, Play::Scissors) => 3 + LOSE,
            (Play::Scissors, Play::Rock) => 1 + WIN,
            (Play::Scissors, Play::Paper) => 2 + LOSE,
            (Play::Scissors, Play::Scissors) => 3 + TIE,
        }
    }).sum();

    return a.to_string();
}

pub fn run_part_2(input: &str) -> String {
    let a: usize = input.lines().map(|l| {
        let left_input: Vec<Play> = l.trim().split_whitespace().take(1).map(|i| i.parse::<Play>().unwrap()).collect();
        let left_play = left_input.first().unwrap();
        let vec: Vec<&str> = l.trim().split_whitespace().collect();
        let right_input = *vec.last().unwrap();

        let right = match right_input {
            "X" => match left_play {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            "Y" => match left_play {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissors => Play::Scissors,
            },
            "Z" => match left_play {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            }
            _ => panic!("invalid input")
        };

        match (left_play, right) {
            (Play::Paper, Play::Rock) => 1 + LOSE,
            (Play::Paper, Play::Paper) => 2 + TIE,
            (Play::Paper, Play::Scissors) => 3 + WIN,
            (Play::Rock, Play::Rock) => 1 + TIE,
            (Play::Rock, Play::Paper) => 2 + WIN,
            (Play::Rock, Play::Scissors) => 3 + LOSE,
            (Play::Scissors, Play::Rock) => 1 + WIN,
            (Play::Scissors, Play::Paper) => 2 + LOSE,
            (Play::Scissors, Play::Scissors) => 3 + TIE,

        }
    }).sum();

    return a.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "A Y
B X
C Z";
        let result = run_part_1(input);
        assert_eq!(result, "15");
    }

    #[test]
    fn it_works_2() {
        let input = "A Y
B X
C Z";
        let result = run_part_2(input);
        assert_eq!(result, "12");
    }
}
