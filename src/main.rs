use std::env;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod solver;

fn main() {
    // Read from stdin
    let args: Vec<String> = env::args().collect();

    // Parse first argument as a uint8
    let day: u8 = args[1].parse().unwrap();

    let solver = get_solver(day).unwrap();
    // read file contents as an array of lines without using include_str
    let input = format!("./inputs/day{:02}.txt", day);
    // rust, read a file as a vector of strings
    let lines = lines_from_file(input);
    println!("Part 1: {}", solver.solve_part_1(lines.clone()));
    println!("Part 2: {}", solver.solve_part_2(lines));
}

fn get_solver(day: u8) -> Option<Box<dyn solver::Solver>> {
    match day {
        1 => Some(Box::new(day01::Day1Solver {})),
        2 => Some(Box::new(day02::Day2Solver {})),
        3 => Some(Box::new(day03::Day3Solver {})),
        4 => Some(Box::new(day04::Day4Solver {})),
        5 => Some(Box::new(day05::Day5Solver {})),
        _ => None,
    }
}

/// Returns a vector of String. The idea is to use this format to consume lines
/// from the files. We can also mock this out by passing Vec<String> to the solutions
/// since they expect this format as well.
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
