use color_eyre;
use std::str::FromStr;

use crate::solver::Solver;

pub struct Day2Solver {}

impl Solver for Day2Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut your_score = 0u32;
        for line in lines {
            let round = Round::from_str(&line).unwrap();
            your_score += round.you.outcome(round.opponent).score() + round.you.score();
        }
        your_score.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        "idk".to_string()
        //todo!()
    }
}

#[derive(Debug, Clone, Copy)]
struct Round {
    you: Choice,
    opponent: Choice,
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.chars();
        // Here the trick is that we take a line and consume the characters on by one. For
        // something really simple where the string format is very simple consuming char one by one
        // is enough.
        let (Some(opponent), Some(' '), Some(you), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(color_eyre::eyre::eyre!("bad"));
        };
        Ok(Self {
            you: you.try_into()?,
            opponent: opponent.try_into()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn outcome(self, opponent: Choice) -> Outcome {
        if self.beats(opponent) {
            Outcome::Win
        } else if opponent.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn beats(self, opponent_move: Choice) -> bool {
        // This checks if the tuple (your move, opponent move) matches
        // any of the following patterns. This allows us to check if you
        // beat your opponent.
        matches!(
            (self, opponent_move),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl TryFrom<char> for Choice {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::lines_from_file;

    #[test]
    fn test_part_1() {
        let solver = Day2Solver {};
        let lines = lines_from_file("./inputs/unit_test/day02.txt");
        assert_eq!(solver.solve_part_1(lines.clone()), "15");
    }
}
