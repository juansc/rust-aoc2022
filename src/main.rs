use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};
use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod solver;
mod utils;

fn main() {
    // Read from stdin
    let args: Vec<String> = env::args().collect();

    // Parse first argument as a uint8
    let day: u8 = args[1].parse().unwrap();

    let solver = get_solver(day).unwrap();
    // read file contents as an array of lines without using include_str
//    let input = format!("./inputs/unit_test/day{:02}.txt", day);
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
        6 => Some(Box::new(day06::Day6Solver {})),
        7 => Some(Box::new(day07::Day7Solver {})),
        8 => Some(Box::new(day08::Day8Solver {})),
        9 => Some(Box::new(day09::Day9Solver {})),
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
