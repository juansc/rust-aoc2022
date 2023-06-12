use std::collections::HashSet;

use crate::solver::Solver;
use crate::utils::grid::{GridCoord};

pub struct Day9Solver {}

impl Solver for Day9Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut rope = Rope::new(2);
        let uniq_pos = simulate_rope(&mut rope, lines);
        format!("{}", uniq_pos)
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let mut rope = Rope::new(10);
        let uniq_pos = simulate_rope(&mut rope, lines);
        format!("{}", uniq_pos)
    }
}

fn simulate_rope(rope: &mut Rope, lines: Vec<String>) -> usize {
    let mut uniq_pos = HashSet::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = match *parts.get(0).unwrap() {
            "R" => RopeMovement::Right,
            "L" => RopeMovement::Left,
            "U" => RopeMovement::Up,
            "D" => RopeMovement::Down,
            _ => {
                println!("Invalid direction: {}", parts.get(0).unwrap());
                panic!("Invalid direction")
            }
        };
        let mut num = parts.get(1).unwrap().parse::<usize>().unwrap();
        while num > 0 {
            rope.update(direction);
            uniq_pos.insert(rope.tail_pos());
            num -= 1;
        }
    }
    uniq_pos.len()
}

struct Rope {
    knots: Vec<GridCoord>,
    length: usize,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut knots = Vec::with_capacity(length);
        for _ in 0..length {
            knots.push((0, 0).into());
        }
        Rope {
            knots,
            length,
        }
    }

    fn update(&mut self, movement: RopeMovement) {
        self.knots[0] = self.knots[0] + movement.new_coord();
        for idx in 1..self.knots.len() {
            let current_knot = self.knots[idx];
            let prev_knot = self.knots[idx - 1];
            if is_touching(current_knot, prev_knot) {
                continue;
            }
            let dist = prev_knot - current_knot;
            let dx = dist.x;
            let dy = dist.y;

            // We will simplify the problem by assuming that we update the tail immediately after any
            // movement. We don't have to handle the case where the head is so far away from the tail
            // that is has to chase it far away. We only have to handle the case where
            // 1. The head is two spaces away either LEFT, RIGHT, UP, or DOWN
            // 2. The head is a chess knight's move away from the tail
            // In the first case we will move the tail one space in the direction of the head.
            // In the second case we need to move the tail one space in the direction of the head
            // in each dimension.
            let new_dx = match dx {
                0 => 0,
                _ => dx / dx.abs(),
            };
            let new_dy = match dy {
                0 => 0,
                _ => dy / dy.abs(),
            };
            self.knots[idx] = current_knot + (new_dx, new_dy).into();
        }
    }

    fn tail_pos(&self) -> GridCoord {
        self.knots[self.length - 1]
    }
}

fn is_touching(head: GridCoord, tail: GridCoord) -> bool {
    // If the points are identical they touch
    if head == tail {
        return true;
    }
    let dist = head - tail;
    // If they are adjacent then they touch
    if dist.x.abs() + dist.y.abs() <= 1 {
        return true;
    }
    // If they are exactly diagonal from each other they are also touching.
    if dist.x.abs() == 1 && dist.y.abs() == 1 {
        return true;
    }
    false
}

#[derive(Clone, Copy)]
enum RopeMovement {
    Left,
    Right,
    Up,
    Down,
}

impl RopeMovement {
    fn new_coord(&self) -> GridCoord {
        match self {
            RopeMovement::Left => { (-1, 0).into() }
            RopeMovement::Right => { (1, 0).into() }
            RopeMovement::Up => { (0, -1).into() }
            RopeMovement::Down => { (0, 1).into() }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day09::Day9Solver;
    use crate::lines_from_file;
    use crate::solver::Solver;

    #[test]
    fn test_part_1() {
        let solver = Day9Solver {};
        let lines = lines_from_file("./inputs/unit_test/day09.txt");
        assert_eq!(solver.solve_part_1(lines), "13")
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day9Solver {};
        let lines = lines_from_file("./inputs/day09.txt");
        assert_eq!(solver.solve_part_1(lines), "6494")
    }

    #[test]
    fn test_part_2() {
        let solver = Day9Solver {};
        let lines: Vec<String> = vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string(),
        ];
        assert_eq!(solver.solve_part_2(lines), "36")
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day9Solver {};
        let lines = lines_from_file("./inputs/day09.txt");
        assert_eq!(solver.solve_part_2(lines), "2691")
    }
}

