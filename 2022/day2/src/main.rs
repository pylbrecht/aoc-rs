use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::ops::Fn;
use std::str::FromStr;

fn main() {
    let rounds = fs::read_to_string("input.txt").unwrap();
    {
        let score = play(&rounds, Part1::score_round);
        println!("{}", score);
    }

    let score = play(&rounds, Part2::score_round);
    println!("{}", score);
}

type Score = usize;

#[derive(Debug, Clone, PartialEq)]
struct ShapeError;

#[derive(Debug, Clone, PartialEq)]
struct OutcomeError;

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Tie,
    Loss,
}

impl FromStr for Outcome {
    type Err = OutcomeError;

    fn from_str(s: &str) -> Result<Self, OutcomeError> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(OutcomeError),
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Rock => match other {
                Self::Rock => Some(Ordering::Equal),
                Self::Paper => Some(Ordering::Less),
                Self::Scissors => Some(Ordering::Greater),
            },
            Self::Paper => match other {
                Self::Rock => Some(Ordering::Greater),
                Self::Paper => Some(Ordering::Equal),
                Self::Scissors => Some(Ordering::Less),
            },
            Self::Scissors => match other {
                Self::Rock => Some(Ordering::Less),
                Self::Paper => Some(Ordering::Greater),
                Self::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl Shape {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn from_str(s: &str) -> Result<Self, ShapeError> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ShapeError),
        }
    }
}

trait ScoreRound {
    fn score_round(round: &str) -> Score;
}

struct Part1;
struct Part2;

impl ScoreRound for Part1 {
    fn score_round(round: &str) -> Score {
        let shapes: Vec<&str> = round.split(" ").collect();
        let shape_opponent = Shape::from_str(shapes[0]).unwrap();
        let shape_myself = Shape::from_str(shapes[1]).unwrap();

        if shape_opponent > shape_myself {
            shape_myself.score()
        } else if shape_opponent < shape_myself {
            shape_myself.score() + 6
        } else {
            shape_myself.score() + 3
        }
    }
}

impl ScoreRound for Part2 {
    fn score_round(round: &str) -> Score {
        let shapes: Vec<&str> = round.split(" ").collect();
        let shape_opponent = Shape::from_str(shapes[0]).unwrap();
        let outcome = Outcome::from_str(shapes[1]).unwrap();

        match outcome {
            Outcome::Loss => shape_opponent.beats().score(),
            Outcome::Tie => shape_opponent.score() + 3,
            Outcome::Win => shape_opponent.beaten_by().score() + 6,
        }
    }
}

fn play<F>(rounds: &str, scorer: F) -> Score
where
    F: Fn(&str) -> Score,
{
    rounds
        .trim_end_matches("\n")
        .split("\n")
        .map(|round| scorer(round))
        .sum()
}


mod test {
    use super::*;

    #[test]
    fn create_shape_from_str() {
        assert_eq!(Shape::from_str("A"), Ok(Shape::Rock));
        assert_eq!(Shape::from_str("X"), Ok(Shape::Rock));
        assert_eq!(Shape::from_str("B"), Ok(Shape::Paper));
        assert_eq!(Shape::from_str("Y"), Ok(Shape::Paper));
        assert_eq!(Shape::from_str("C"), Ok(Shape::Scissors));
        assert_eq!(Shape::from_str("Z"), Ok(Shape::Scissors));
    }

    #[test]
    fn compare_shapes() {
        assert!(Shape::Rock > Shape::Scissors);
        assert!(Shape::Paper > Shape::Rock);
        assert!(Shape::Scissors > Shape::Paper);
    }

    #[test]
    fn get_score_for_shape() {
        assert_eq!(Shape::Rock.score(), 1);
        assert_eq!(Shape::Paper.score(), 2);
        assert_eq!(Shape::Scissors.score(), 3);
    }

    #[test]
    fn test_play_round() {
        let round = "A Y";
        assert_eq!(Part1::score_round(round), 8);
    }

    #[test]
    fn play_rounds() {
        let rounds = "A Y\nB X\nC Z\n";
        assert_eq!(play(rounds, Part1::score_round), 15);
    }

    #[test]
    fn create_outcome_from_str() {
        assert_eq!(Outcome::from_str("X"), Ok(Outcome::Loss));
        assert_eq!(Outcome::from_str("Y"), Ok(Outcome::Tie));
        assert_eq!(Outcome::from_str("Z"), Ok(Outcome::Win));
    }

    #[test]
    fn test_play_round_part2() {
        let round = "C Z";
        assert_eq!(Part2::score_round(round), 7);
    }

    #[test]
    fn play_rounds_part2() {
        let rounds = "A Y\nB X\nC Z\n";
        assert_eq!(play(rounds, Part2::score_round), 12);
    }
}
