use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    str::FromStr,
};

use crate::solver::Solver;

pub struct Day3Solver {}

impl Solver for Day3Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut priority = 0usize;
        for line in lines {
            let line = line.trim();
            let sack = Rucksack::from_str(line).unwrap();
            if let Some(k) = sack.find_common_item() {
                let item_priority = k.priority();
                priority += item_priority;
                continue;
            }
        }
        priority.to_string()
    }

    // This is my ugly, initial approach.
    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut priority = 0usize;
        let mut lines = lines;
        lines.retain(|l| !l.trim().is_empty());
        let mut deque: VecDeque<String> = VecDeque::from(lines);
        if deque.len() % 3 != 0 {
            println!("deque has {} elements", deque.len());
            panic!("expected a multiple of 3 rucksacks")
        }
        loop {
            let sack1 = Rucksack::from_str(deque.pop_front().unwrap().trim())
                .unwrap()
                .keys();
            let sack2 = Rucksack::from_str(deque.pop_front().unwrap().trim())
                .unwrap()
                .keys();
            let sack3 = Rucksack::from_str(deque.pop_front().unwrap().trim())
                .unwrap()
                .keys();
            let mut all_keys: Vec<Item> = sack1
                .iter()
                .copied()
                .filter(|i| sack2.contains(i) && sack3.contains(i))
                .collect();
            if all_keys.len() != 1 {
                panic!("There were no common keys")
            }
            priority += all_keys.pop().unwrap().priority();
            if deque.is_empty() {
                break;
            }
        }
        priority.to_string()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
// TODO: We could add an init that verifies that only a-zA-Z are allowed,
// but for the purpose of this exercise I chose to ignore that requirement.
struct Item(char);

impl Item {
    // According to the prompt, the characters have a mapping of
    // a-z = 1-26
    // A-Z = 27-52
    // Here we do some math with the code points to do this in a legible way
    fn priority(&self) -> usize {
        let val = self.0 as u8;
        let val = if (b'a'..=b'z').contains(&val) {
            val - (b'a') + 1
        } else {
            val - (b'A') + 1 + 26
        };
        val as usize
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Rucksack {
    first_compartment: HashMap<Item, usize>,
    second_compartment: HashMap<Item, usize>,
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
        let mut first_comp: HashMap<Item, usize> = HashMap::new();
        let mut second_comp: HashMap<Item, usize> = HashMap::new();
        for c in first_desc.chars() {
            *first_comp.entry(Item(c)).or_insert(0) += 1;
        }
        for c in second_desc.chars() {
            *second_comp.entry(Item(c)).or_insert(0) += 1;
        }
        Ok(Self {
            first_compartment: first_comp,
            second_compartment: second_comp,
        })
    }
}

impl Rucksack {
    fn find_common_item(&self) -> Option<Item> {
        for k in self.first_compartment.keys() {
            if self.second_compartment.contains_key(k) {
                let x = *k;
                return Some(x);
            }
        }
        None
    }

    fn keys(&self) -> HashSet<Item> {
        let mut set = HashSet::new();
        set.extend(self.first_compartment.keys());
        set.extend(self.second_compartment.keys());
        set
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
        assert_eq!(solver.solve_part_1(lines), "157");
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/day03.txt");
        assert_eq!(solver.solve_part_1(lines), "8109");
    }

    #[test]
    fn test_part_2() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/unit_test/day03.txt");
        assert_eq!(solver.solve_part_2(lines), "70");
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day3Solver {};
        let lines = lines_from_file("./inputs/day03.txt");
        assert_eq!(solver.solve_part_2(lines), "2738");
    }
}
