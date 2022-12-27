use std::str::FromStr;

use crate::solver::Solver;

pub struct Day2Solver {}

impl Solver for Day2Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut your_score = 0u32;
        for line in lines {
            let round = Part1Round::from_str(&line).unwrap();
            your_score += round.you.outcome(round.opponent).score() + round.you.score();
        }
        your_score.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut your_score = 0u32;
        for line in lines {
            // At this point we've fully parsed a round. Since we have the desired outcome
            // we know the points we should get for the match -- now we just need to figure
            // out what move we should throw.
            let round = Part2Round::from_str(&line).unwrap();
            your_score += round.desired_outcome.score();
            let desired_move = match round.desired_outcome {
                Outcome::Win => round.opponent.get_losing_choice(),
                Outcome::Loss => round.opponent.get_winning_choice(),
                Outcome::Draw => round.opponent.get_draw_choice(),
            };
            your_score += desired_move.score();
        }
        your_score.to_string()
    }
}

// Part1Round encodes the information in part 1: What moves you and your opponent
// will do.
#[derive(Debug, Clone, Copy)]
struct Part1Round {
    you: Choice,
    opponent: Choice,
}

impl FromStr for Part1Round {
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
struct Part2Round {
    desired_outcome: Outcome,
    opponent: Choice,
}

impl FromStr for Part2Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.chars();
        // Here the trick is that we take a line and consume the characters on by one. For
        // something really simple where the string format is very simple consuming char one by one
        // is enough.
        let (Some(opponent), Some(' '), Some(desired_outcome), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
            return Err(color_eyre::eyre::eyre!("bad"));
        };
        Ok(Self {
            desired_outcome: desired_outcome.try_into()?,
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

    const ALL_CHOICES: [Self; 3] = [Self::Rock, Self::Paper, Self::Scissors];

    fn get_winning_choice(self) -> Self {
        Self::ALL_CHOICES
            .iter()
            .copied()
            .find(|&c| self.beats(c))
            .expect("there should be one choice we can beat")
    }

    fn get_losing_choice(self) -> Self {
        Self::ALL_CHOICES
            .iter()
            .copied()
            .find(|c| c.beats(self))
            .expect("there should be one choice can beat us")
    }

    fn get_draw_choice(self) -> Self {
        self
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

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {value:?}")),
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
        assert_eq!(solver.solve_part_1(lines), "15");
    }

    #[test]
    fn test_part_2() {
        let solver = Day2Solver {};
        let lines = lines_from_file("./inputs/unit_test/day02.txt");
        assert_eq!(solver.solve_part_2(lines), "12");
    }
}
