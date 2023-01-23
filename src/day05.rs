use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::solver::Solver;

pub struct Day5Solver {}

impl Day5Solver {
    fn move_stacks_with_mode(&self, lines: Vec<String>, stack_mode: StackOrder) -> String {
        let mut lines = lines.into_iter();
        // This will advance the iterator until we are not able to parse.
        let mut crate_lines: Vec<_> = (&mut lines)
            .map_while(|line| {
                all_consuming(parse_crate_line)(&line)
                    .ok()
                    // Recall that the output of parse_crate_line is a IResult<&str, Crate>. The
                    // call to ok() returns an Option<(&str, rate)>. Then we use map to grab just
                    // the crate.
                    .map(|(_, cl)| cl)
            })
            .collect();
        // The stacks were given from the top down. Reverse the lines so that the crates can be
        // built from the bottom up.
        crate_lines.reverse();
        let mut crate_stacks = CrateStacks::get_new_stacks(&crate_lines);

        assert!(lines.next().unwrap().is_empty());

        let instructions: Vec<_> = (&mut lines)
            .map_while({
                |line| {
                    all_consuming(parse_instruction)(&line)
                        .ok()
                        .map(|(_, instr)| instr)
                }
            })
            .collect();

        for ins in instructions {
            crate_stacks.apply_instruction(&ins, stack_mode);
        }

        let top_row = crate_stacks
            .get_top_crates_for_stack()
            .iter()
            .map(|my_crate| my_crate.unwrap_or_else(|| Crate(' ')).0.to_string())
            .reduce(|acc, e| acc + &e).unwrap();

        top_row
    }
}

impl Solver for Day5Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        self.move_stacks_with_mode(lines, StackOrder::Lifo)
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        self.move_stacks_with_mode(lines, StackOrder::Fifo)
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dest: usize,
}

// Taken from fasterthanlime's advent of code.
fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dest)| Instruction {
            quantity,
            src,
            dest,
        },
    )(s)
}

#[derive(Debug, Clone)]
struct CrateStack(Vec<Crate>);

impl CrateStack {
    fn add_crates(&mut self, other: &CrateStack) {
        for c in &other.0 {
            self.0.push(*c);
        }
    }

    // remove_crates removes num crates from the top of the stack,
    // reverses the removed stack, and returns it. If num is larger
    // the size of this stack then the operation fails.
    fn remove_crates(&mut self, num: usize, stack_order: StackOrder) -> Option<CrateStack> {
        if num > self.0.len() {
            return None;
        }
        let crate_copy = self.0.clone();
        let (remaining, other) = crate_copy.split_at(self.0.len() - num);

        self.0 = remaining.to_vec();
        let mut other = other.to_vec();
        match stack_order {
            StackOrder::Lifo => {other.reverse()}
            _ => {},
        }
        Some(CrateStack(other))
    }
}

#[derive(Clone, Copy)]
enum StackOrder {
    Fifo,
    Lifo,
}

#[derive(Debug, Clone)]
struct CrateStacks(Vec<CrateStack>);

impl CrateStacks {
    fn get_new_stacks(lines: &Vec<Vec<Option<Crate>>>) -> Self {
        let mut stacks = vec![CrateStack(vec![]); lines.first().unwrap().len()];
        for row in lines {
            for (i, col) in row.iter().enumerate() {
                match col {
                    Some(c) => stacks[i].0.push(*c),
                    None => {}
                }
            }
        }
        Self(stacks)
    }

    fn apply_instruction(&mut self, instr: &Instruction, stack_order: StackOrder) {
        let crates = self
            .0
            .get_mut(instr.src)
            .unwrap()
            .remove_crates(instr.quantity, stack_order)
            .unwrap();
        self.0.get_mut(instr.dest).unwrap().add_crates(&crates);
    }

    fn get_top_crates_for_stack(&self) -> Vec<Option<Crate>> {
        let mut my_crates = vec![];
        for stack in self.0.iter() {
            let crate_contents = match stack.0.last() {
                Some(c) => Some(c.clone()),
                None => None,
            };
            my_crates.push(crate_contents);
        }
        return my_crates;
    }
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);

// This function consumes a &str, and emits the remaining str and the crate that
// lives there. This ignores errors.
fn parse_crate(s: &str) -> IResult<&str, Crate> {
    // This is obvious: Consume a [, grab and return the next char, and consume ]. In
    // this context, consume means to discard.
    let parser = delimited(tag("["), take(1_usize), tag("]"));
    // This function is gnarly!
    //
    // pub fn map<I, O1, O2, E, F, G>(
    //     parser: F,
    //     f: G
    // ) -> impl FnMut(I) -> IResult<I, O2, E>
    // where
    //     F: Parser<I, O1, E>,
    //     G: FnMut(O1) -> O2,
    //
    // This function has the following players:
    //  I - No constraints, but it is the thing that we are consuming (&str)
    //  O1, O2 - They are the preimage and image of G.
    //  E - No constraints, but given its potition in the IResult, it's an error type
    //  F - Parser type. Note that it takes in I, O1 (input), and has an associated error type.
    //  G - Function that converts O1 to O2. In this case it can mutate O1 as well, since it's
    //      FnMut.
    //
    // From the docs: `map` maps a function on the result of a parser.
    // So, this function runs the parser f on s, and then executes char_fn on the result.
    map(parser, |ss: &str| Crate(ss.chars().next().unwrap()))(s)
}

fn parse_hole(s: &str) -> IResult<&str, ()> {
    // drop is a built-in function that takes a value and returns nothing.
    map(tag("   "), drop)(s)
}

fn parse_crate_or_hole(s: &str) -> IResult<&str, Option<Crate>> {
    // alt is a function that takes a list of parsers and tries them in order.
    // If any of them succeed, it returns the result of that parser.
    // If none of them succeed, it returns an error.
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(s)
}

fn parse_crate_line(s: &str) -> IResult<&str, Vec<Option<Crate>>> {
    // Here we consume a line and produce a vector where
    // each element is a column.

    // Here we say that i is mutable but c is not.
    let (mut i, c) = parse_crate_or_hole(s)?;
    let mut v = vec![c];

    loop {
        // The preceded parser consumes and ignores the first element (a space), then
        // returns the value from the second parser. You can read this as:
        // "Consuem a space, then a crate or a hole".
        // This whole operation can fail, so the opt function will return a None instead
        // of an error.
        let (next_i, maybe_a_crate) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_a_crate {
            // Found another crate, keep it going!
            Some(a_crate) => v.push(a_crate),
            // We are done!
            None => break,
        }
        i = next_i;
    }

    // We have finished and can return the remaining string and vector
    Ok((i, v))
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::lines_from_file;

    #[test]
    fn test_part_1() {
        let solver = Day5Solver{};
        let lines = lines_from_file("./inputs/unit_test/day05.txt");
        assert_eq!(solver.solve_part_1(lines), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let solver = Day5Solver{};
        let lines = lines_from_file("./inputs/unit_test/day05.txt");
        assert_eq!(solver.solve_part_2(lines), "MCD");
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day5Solver{};
        let lines = lines_from_file("./inputs/day05.txt");
        assert_eq!(solver.solve_part_1(lines), "TLNGFGMFN");
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day5Solver{};
        let lines = lines_from_file("./inputs/day05.txt");
        assert_eq!(solver.solve_part_2(lines), "FGLQJCMBD");
    }
}
