use color_eyre;
use std::str::FromStr;

use crate::solver::Solver;

pub struct Day2Solver {}

impl Solver for Day2Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut your_score = 0u32;
        for line in lines {
            // split string into a vector by space, then get first element
            let mut parts = line.split(" ");
            let opponent = parts.next().unwrap();
            let you = parts.next().unwrap();
            let opponent_choice = parse(opponent);
            let you_choice = parse(you);
            your_score += score(you_choice, opponent_choice);
        }
        your_score.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        todo!()
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

fn parse(s: &str) -> Choice {
    match s {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => {
            panic!("Invalid choice")
        }
    }
}

fn score(you: Choice, opponent: Choice) -> u32 {
    let choice_score: u32 = match you {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let outcome_score = match you {
        Choice::Rock => match opponent {
            Choice::Rock => 3,
            Choice::Paper => 0,
            Choice::Scissors => 6,
        },
        Choice::Paper => match opponent {
            Choice::Rock => 6,
            Choice::Paper => 3,
            Choice::Scissors => 0,
        },
        Choice::Scissors => match opponent {
            Choice::Rock => 0,
            Choice::Paper => 6,
            Choice::Scissors => 3,
        },
    };
    choice_score + outcome_score
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
