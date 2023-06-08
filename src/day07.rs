// Implementing a tree in Rust is non-trivial. I'd like to tackle
// these problems as if this was a work assignment, pulling in
// crates if necessary and implementing the functionality if it's
// fun or worthwhile.
use std::fmt::{Debug, Formatter};

use indextree::{Arena, NodeId};
use nom::{Finish, IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{all_consuming, map};
use nom::sequence::{preceded, separated_pair};

use crate::solver::Solver;

fn parse_path(i: &str) -> IResult<&str, String> {
    // This says "grab one character at a time until it doesn't match the given condition, then
    // convert into SOMETHING". That SOMETHING is inferred from the function signature, and that
    // is a String.
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    // Tag consumes a "ls" string and returns what's left. This errors if there is no "ls"
    map(
        tag("ls"),
        |_| Ls,
    )(i)
}

const LARGE_DIR_THRESHOLD_SIZE: usize = 100_000;
const MINIMUM_INSTALL_SIZE: usize = 30_000_000;
const TOTAL_DISK_SIZE: usize = 70_000_000;


#[derive(Debug)]
struct Cd(String);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    // This says "make sure that the input is preceded by a cd, then parse using parse_path"
    map(
        preceded(tag("cd "), parse_path),
        Cd,
    )(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command_line(i: &str) -> IResult<&str, Command> {
    // If these functions succeed they return the remaining input and the parsed output.
    // In this case we don't care about the parsed_output.
    let (i, _) = tag("$ ")(i)?;
    alt(
        (
            // Pretty neat! Into::into allows you to take implicit A->B and it's the same as
            // calling B::From(A)
            map(parse_ls, Into::into),
            map(parse_cd, Into::into),
        )
    )(i)
}

#[derive(Debug)]
enum LsEntry {
    Dir(String),
    File(u64, String),
}

/// parse_ls_entry parses the output of running ls
fn parse_ls_lines(i: &str) -> IResult<&str, LsEntry> {
    let parse_file_line = map(
        // Use two parsing functions separated by some other combinator, in this case an empty space
        separated_pair(
            nom::character::complete::u64,
            tag(" "),
            parse_path,
        ),
        |(size, path)| LsEntry::File(size, path),
    );
    let parse_dir_line = map(
        preceded(
            tag("dir "),
            parse_path,
        ),
        LsEntry::Dir,
    );
    // Try parsing first as a file, then as a directory
    alt((parse_file_line, parse_dir_line))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    LsEntry(LsEntry),
}

fn parse_lines(i: &str) -> IResult<&str, Line> {
    alt(
        (
            map(parse_command_line, Line::Command),
            map(parse_ls_lines, Line::LsEntry),
        )
    )(i)
}

struct FsEntry {
    name: String,
    size: Option<usize>,
}

impl FsEntry {
    fn is_dir(&self) -> bool {
        self.size.is_none()
    }
}

impl Debug for FsEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.size {
            None => { writeln!(f, "{}/", self.name)?; }
            Some(size) => { writeln!(f, "{} - {}", self.name, size)?; }
        }
        Ok(())
    }
}

pub struct Day7Solver {}

fn tree_from_cmd_lines(lines: Vec<Line>) -> (Arena<FsEntry>, NodeId) {
    let mut tree: Arena<FsEntry> = Arena::new();
    let mut id = tree.new_node(FsEntry {
        name: "/".to_string(),
        size: None,
    });
    let root_id = id;
    for line in lines {
        match line {
            Line::Command(cmd) => {
                match cmd {
                    // Do nothing on the ls command since it yields no new information
                    Command::Ls => {}
                    // On Cd either move up or create an add new child node
                    Command::Cd(location) => match location.as_str() {
                        "/" => {}
                        ".." => {
                            id = tree[id].parent().unwrap();
                        }
                        _ => {
                            for child in id.children(&tree) {
                                if tree[child].get().name == location {
                                    id = child;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Line::LsEntry(entry) => {
                match entry {
                    LsEntry::Dir(dir) => {
                        // Problems here: There is no guarantee that this node doesn't
                        // already exist. We can check that this node doesn't already
                        // have a child.
                        let node = tree.new_node(FsEntry {
                            name: dir.to_string(),
                            size: None,
                        }
                        );
                        id.append(node, &mut tree);
                    }
                    LsEntry::File(size, name) => {
                        let node = tree.new_node(FsEntry {
                            name,
                            size: Some(size as usize),
                        }
                        );
                        id.append(node, &mut tree);
                    }
                }
            }
        }
    }
    println!("{:?}", root_id.debug_pretty_print(&tree));
    (tree, root_id)
}

impl Solver for Day7Solver {
    // This solution involves building the FS tree and navigating it and finding all directories
    // that are smaller than the given size.
    fn solve_part_1(&self, lines: Vec<String>) -> String {
        let lines = lines.into_iter()
            .map(|l| all_consuming(parse_lines)(&l).finish().unwrap().1).collect();

        let (tree, root_id) = tree_from_cmd_lines(lines);

        let mut cum_sum_small_dirs_size = 0;
        for node in root_id.descendants(&tree) {
            if !tree[node].get().is_dir() {
                continue;
            }
            let size = get_size_of_tree(node, &tree);
            if size <= LARGE_DIR_THRESHOLD_SIZE {
                cum_sum_small_dirs_size += size;
            }
        }
        format!("{}", cum_sum_small_dirs_size)
    }

    // For this solution we iterate through all the directories and finding the smallest directory
    // that will delete the necessary amount.
    fn solve_part_2(&self, lines: Vec<String>) -> String {
        let lines = lines.into_iter()
            .map(|l| all_consuming(parse_lines)(&l).finish().unwrap().1).collect();

        let (tree, root_id) = tree_from_cmd_lines(lines);
        let current_size = get_size_of_tree(root_id, &tree);
        let mut smallest_dir_size = usize::MAX;
        for node in root_id.descendants(&tree) {
            if !tree[node].get().is_dir() {
                continue;
            }
            let size = get_size_of_tree(node, &tree);
            if TOTAL_DISK_SIZE - (current_size - size) >= MINIMUM_INSTALL_SIZE && size < smallest_dir_size {
                smallest_dir_size = size;
            }
        }
        format!("{}", smallest_dir_size)
    }
}

fn get_size_of_tree(node_id: NodeId, arena: &Arena<FsEntry>) -> usize {
    let entry = arena[node_id].get();
    if !entry.is_dir() {
        return entry.size.unwrap();
    }
    let mut tree_size = 0;
    for child in node_id.children(arena) {
        tree_size += get_size_of_tree(child, arena);
    }
    tree_size
}

#[cfg(test)]
mod test {
    use crate::day07::Day7Solver;
    use crate::lines_from_file;
    use crate::solver::Solver;

    #[test]
    fn test_part_1() {
        let solver = Day7Solver {};
        let lines = lines_from_file("./inputs/unit_test/day07.txt");
        assert_eq!(solver.solve_part_1(lines), "95437")
    }

    #[test]
    fn test_part_1_full() {
        let solver = Day7Solver {};
        let lines = lines_from_file("./inputs/day07.txt");
        assert_eq!(solver.solve_part_1(lines), "1491614")
    }

    #[test]
    fn test_part_2() {
        let solver = Day7Solver {};
        let lines = lines_from_file("./inputs/unit_test/day07.txt");
        assert_eq!(solver.solve_part_2(lines), "24933642")
    }

    #[test]
    fn test_part_2_full() {
        let solver = Day7Solver {};
        let lines = lines_from_file("./inputs/day07.txt");
        assert_eq!(solver.solve_part_2(lines), "6400111")
    }
}
