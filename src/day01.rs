use crate::solver;

pub struct Day1Solver {}

impl solver::Solver for Day1Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut max_val: u32 = 0;
        let mut current: u32 = 0;
        for line in lines {
            if line.is_empty() {
                if current > max_val {
                    max_val = current;
                }
                current = 0;
                continue;
            }
            current += line.parse::<u32>().unwrap();
        }
        if current > max_val {
            max_val = current;
        }
        max_val.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut calories: Vec<u32> = vec![];
        let mut current: u32 = 0;
        for line in lines {
            if line.is_empty() {
                calories.push(current);
                current = 0;
                continue;
            }
            current += line.parse::<u32>().unwrap();
        }
        calories.sort();
        // string with greeting for ssh login
        calories.iter().take(3).sum::<u32>().to_string()
    }
}
