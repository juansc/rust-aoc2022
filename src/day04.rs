use crate::solver::Solver;
use std::ops::RangeInclusive;

pub struct Day4Solver {}

impl Solver for Day4Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut counter = 0usize;
        for line in lines {
            let line = line.trim();
            let ranges: Vec<&str> = line.split(',').collect();
            let range_a = ranges.first().unwrap().split('-').collect::<Vec<&str>>();
            let min_range_a = range_a.first().unwrap().parse::<usize>().unwrap();
            let max_range_a = range_a.last().unwrap().parse::<usize>().unwrap();

            let range_b = ranges.get(1).unwrap().split('-').collect::<Vec<&str>>();
            let min_range_b = range_b.first().unwrap().parse::<usize>().unwrap();
            let max_range_b = range_b.last().unwrap().parse::<usize>().unwrap();

            let range_a = min_range_a..=max_range_a;
            let range_b = min_range_b..=max_range_b;
            if contains_latter(&range_a, &range_b) || contains_latter(&range_b, &range_a) {
                counter += 1;
            }
        }
        counter.to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut counter = 0usize;
        for line in lines {
            let line = line.trim();
            let ranges: Vec<&str> = line.split(',').collect();
            let range_a = ranges.first().unwrap().split('-').collect::<Vec<&str>>();
            let min_range_a = range_a.first().unwrap().parse::<usize>().unwrap();
            let max_range_a = range_a.last().unwrap().parse::<usize>().unwrap();

            let range_b = ranges.get(1).unwrap().split('-').collect::<Vec<&str>>();
            let min_range_b = range_b.first().unwrap().parse::<usize>().unwrap();
            let max_range_b = range_b.last().unwrap().parse::<usize>().unwrap();

            let range_a = min_range_a..=max_range_a;
            let range_b = min_range_b..=max_range_b;
            if overlaps_latter(&range_a, &range_b) || overlaps_latter(&range_b, &range_a) {
                counter += 1;
            }
        }
        counter.to_string()
    }
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
