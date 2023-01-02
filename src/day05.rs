use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    IResult,
};

use crate::solver::Solver;

pub struct Day5Solver {}

impl Solver for Day5Solver {
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let mut crate_lines = vec![];
        for line in lines {
            // All consuming returns an error if the input is not fully consumed. This is to make
            // sure that our parsers don't accidentally ignore parts of the line.
            if let Ok((_rest, crate_line)) = all_consuming(parse_crate_line)(&line) {
                crate_lines.push(crate_line);
            }
        }
        // The stacks were given from the top down. Reverse the lines so that the crates can be
        // built from the bottom up.
        crate_lines.reverse();
        let mut crate_stacks = CrateStacks::get_new_stacks(&crate_lines);
        for stack in &crate_stacks.0 {
            println!("{:?}", stack);
        }
        "idk".to_string()
    }

    fn solve_part_2(&self, lines: Vec<String>) -> String {
        "idk".to_string()
    }
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
    fn remove_crates(&mut self, num: usize) -> Option<CrateStack> {
        if num > self.0.len() {
            return None;
        }
        let crate_copy = self.0.clone();
        let (remaining, other) = crate_copy.split_at(self.0.len() - num);

        self.0 = remaining.to_vec();
        let mut other = other.to_vec();
        other.reverse();
        Some(CrateStack(other))
    }
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
