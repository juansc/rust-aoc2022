use std::fmt;

pub trait Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String;
    fn solve_part_2(&self, lines: Vec<String>) -> String;
}
