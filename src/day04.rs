use crate::solver::Solver;
use std::ops::RangeInclusive;

use itertools::Itertools;

pub struct Day4Solver {}

impl Solver for Day4Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        get_ranges(&lines)
            .iter()
            .filter(|(a, b)| contains_latter(a, b) || contains_latter(b, a))
            .count()
            .to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        get_ranges(&lines)
            .iter()
            .filter(|(a, b)| overlaps_latter(a, b) || overlaps_latter(b, a))
            .count()
            .to_string()
    }
}

// The following functions parses a Vec of String and returns a Vec of InclusiveRange<u32>. Each
// line is of the form "min-max", where min and max are u32.
fn get_ranges(lines: &[String]) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    lines
        // For each line...
        .iter()
        // Run a mapping function...
        .map(|line| {
            // That splits the line into two strings...
            line.split(',')
                // For each string, split into two using the hyphen, parse each element into a u32,
                // and collect into a tuple of (u32, u32). Then convert each tuple into a range
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("expected a u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("expected each range to be of the form number-number")
                })
                // At this point we have a bunch of RangeInclusives. We collect them into pairs
                .collect_tuple::<(RangeInclusive<_>, RangeInclusive<_>)>()
                // This makes sure that all our Option<X> are actually Some(x). That way we don't
                // need to filter out the failures. We _probably_ should, but this is meant to be a
                // quick exercise.
                .expect("this should succeed")
        })
        // Collect the results into a Vec. Here the type can be inferred as RangeInclusive<_>,
        // and that in turn was inferred to be RangeInclusive<u32>.
        .collect::<Vec<_>>()
}

fn contains_latter<T: PartialOrd>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> bool {
    range1.start() <= range2.start() && range2.end() <= range1.end()
}

fn overlaps_latter<T: PartialOrd>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> bool {
    range1.start() <= range2.end() && range2.start() <= range1.start()
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::lines_from_file;

    #[test]
    fn test_part_1() {
        let solver = Day4Solver {};
        let lines = lines_from_file("./inputs/unit_test/day04.txt");
        assert_eq!(solver.solve_part_1(lines), "2");
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day4Solver {};
        let lines = lines_from_file("./inputs/day04.txt");
        assert_eq!(solver.solve_part_1(lines), "540");
    }

    #[test]
    fn test_part_2() {
        let solver = Day4Solver {};
        let lines = lines_from_file("./inputs/unit_test/day04.txt");
        assert_eq!(solver.solve_part_2(lines), "4");
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day4Solver {};
        let lines = lines_from_file("./inputs/day04.txt");
        assert_eq!(solver.solve_part_2(lines), "872");
    }
}
