use std::{collections::HashMap, str::FromStr};

use color_eyre;

use crate::solver::Solver;

pub struct Day3Solver {}

impl Solver for Day3Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut priority = 0usize;
        for line in lines {
            let mut found_dupe = false;
            let line = line.trim();
            let sack = Rucksack::from_str(&line).unwrap();
            for k in sack.first_compartment.keys() {
                if sack.second_compartment.contains_key(k) {
                    let val = *k as u8;
                    // a-z are 97-122 as u8. Shift so that a-z are 1-26
                    // a-z are 65-90 as u8. Shift so that a-z are 27-52
                    let val = if ('a' as u8) <= val && val <= ('z' as u8) {
                        val - ('a' as u8) + 1
                    } else {
                        val - ('A' as u8) + 1 + 26
                    };
                    println!("{}={}", &k, val);
                    priority += val as usize;
                    found_dupe = true;
                    continue;
                }
            }
            if !found_dupe {
                println!("could not find dupe for line {}", line)
            }
        }
        priority.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        "idk".to_string()
    }
}

struct Rucksack {
    first_compartment: HashMap<char, usize>,
    second_compartment: HashMap<char, usize>,
}

impl FromStr for Rucksack {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let length = s.len();
        if length % 2 == 1 {
            return Err(color_eyre::eyre::eyre!(
                "rucksack string description must have an even number of characters"
            ));
        }
        let (first_desc, second_desc) = (&s[..length / 2], &s[length / 2..]);
        let mut first_comp: HashMap<char, usize> = HashMap::new();
        let mut second_comp: HashMap<char, usize> = HashMap::new();
        for c in first_desc.chars() {
            *first_comp.entry(c).or_insert(0) += 1;
        }
        for c in second_desc.chars() {
            *second_comp.entry(c).or_insert(0) += 1;
        }
        Ok(Self {
            first_compartment: first_comp,
            second_compartment: second_comp,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::lines_from_file;

    #[test]
    fn test_part_1() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/unit_test/day03.txt");
        assert_eq!(solver.solve_part_1(lines.clone()), "157");
    }

    #[test]
    fn test_part_2() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/unit_test/day03.txt");
        assert_eq!(solver.solve_part_2(lines.clone()), "12");
    }
}
