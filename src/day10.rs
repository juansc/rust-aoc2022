use std::collections::VecDeque;

use nom::{Finish, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map, value};
use nom::sequence::preceded;

use crate::solver::Solver;

pub struct Day10Solver;

impl Solver for Day10Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let instructions: VecDeque<Instruction> = lines.iter().map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1).collect();
        let mut cpu = Cpu::new(instructions);
        let mut signal = 0i32;
        while cpu.advance() {
            let cycle = cpu.read_next_cycle();
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    signal += (cycle as i32) * cpu.read_register();
                }
                _ => {}
            }
        }
        format!("{}", signal)
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let instructions: VecDeque<Instruction> = lines.iter().map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1).collect();
        let mut cpu = Cpu::new(instructions);
        let mut crt = Crt::new();
        while cpu.advance() {
            let register = cpu.read_register();
            // The CPU reports the next cycle that it will execute
            let cycle = cpu.read_last_finished_cycle();
            let col_pixel_col = cycle % 40;
            if (register - (col_pixel_col as i32)).abs() <= 1 {
                crt.set_pixel(cycle);
            }
        }
        crt.display()
    }
}

struct Cpu {
    register: i32,
    cycle_num: usize,
    instructions: VecDeque<Instruction>,
    current_instruction: Instruction,
    cycle_instruction_finished: usize,
    instructions_completed: usize,
}

struct Crt {
    pixels: Vec<bool>,
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: vec![false; 240],
        }
    }

    fn set_pixel(&mut self, idx: usize) {
        self.pixels[idx] = true
    }

    fn display(&self) -> String {
        let mut out = "".to_string();
        for row in 0..6 {
            let mut pixel_row = "".to_string();
            for col in 0..40 {
                pixel_row += match self.pixels[row * 40 + col] {
                    true => { "#" }
                    false => { "." }
                };
            }
            out = format!("{}{}\n", out, pixel_row);
        }
        out
    }
}

impl Cpu {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            register: 1,
            cycle_num: 0,
            instructions,
            current_instruction: Instruction::Noop,
            // The default instruction of noop finishes instantly, so we'll
            // load the user defined instructions next.
            cycle_instruction_finished: 1,
            instructions_completed: 0,
        }
    }

    // advance will advance the CPU by one cycle and returns an indicator as to whether it
    // can still continue
    fn advance(&mut self) -> bool {
        if self.instructions.len() == 0 {
            return false;
        }
        // Start the cycle.
        // See if we have any pending instructions. If the one that we've loaded
        // is pending, decrease the ttl. If it's zero, complete the operation and
        // load the next instruction.
        self.cycle_num += 1;

        // If the current instruction is not done leave
        if self.cycle_num != self.cycle_instruction_finished {
            return true;
        }
        self.instructions_completed += 1;

        // The instruction is finished, so wrap up its execution.
        match self.current_instruction {
            Instruction::Noop => {}
            Instruction::AddX(x) => {
                self.register += x;
            }
        }
        self.current_instruction = self.instructions.pop_front().unwrap();

        // Set the timers for when it will finish
        self.cycle_instruction_finished = match self.current_instruction {
            Instruction::Noop => self.cycle_num + 1,
            Instruction::AddX(_) => self.cycle_num + 2
        };

        // We have more cycles left
        true
    }

    fn read_register(&self) -> i32 {
        self.register
    }

    fn read_next_cycle(&self) -> usize {
        self.cycle_num
    }

    fn read_last_finished_cycle(&self) -> usize {
        self.cycle_num - 1
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn parse(i: &str) -> IResult<&str, Self> {
        let noop = tag("noop");
        let addx = preceded(tag("addx "), nom::character::complete::i32);
        alt((
            // If we match Noop, just return the Noop variant
            value(Self::Noop, noop),
            // If preceded by addx, apply the complete i32 parser to it and pass it to Self::AddX
            map(addx, Self::AddX))
        )(i)
    }
}

#[cfg(test)]
mod test {
    use crate::day10::Day10Solver;
    use crate::lines_from_file;
    use crate::solver::Solver;

    #[test]
    fn test_part_1() {
        let solver = Day10Solver {};
        let lines = lines_from_file("./inputs/unit_test/day10.txt");
        assert_eq!(solver.solve_part_1(lines), "13140")
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day10Solver {};
        let lines = lines_from_file("./inputs/day10.txt");
        assert_eq!(solver.solve_part_1(lines), "14760")
    }

    #[test]
    fn test_part_2() {
        let solver = Day10Solver {};
        let lines = lines_from_file("./inputs/unit_test/day10.txt");
        let msg = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ].join("\n") + "\n";
        assert_eq!(solver.solve_part_2(lines), msg)
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day10Solver {};
        let lines = lines_from_file("./inputs/day10.txt");
        // EFGERURE
        let msg = vec![
            "####.####..##..####.###..#..#.###..####.",
            "#....#....#..#.#....#..#.#..#.#..#.#....",
            "###..###..#....###..#..#.#..#.#..#.###..",
            "#....#....#.##.#....###..#..#.###..#....",
            "#....#....#..#.#....#.#..#..#.#.#..#....",
            "####.#.....###.####.#..#..##..#..#.####.",
        ].join("\n") + "\n";
        assert_eq!(solver.solve_part_2(lines), msg)
    }
}


