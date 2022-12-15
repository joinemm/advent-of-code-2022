use core::panic;
use itertools::Itertools;
use slab_tree::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Directory {
    name: String,
    files: HashSet<(String, u32)>,
}

struct State {
    current_dir: NodeId,
}

const FS_SIZE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;

fn dir_size(node: NodeRef<Directory>, dirs: &mut Vec<u32>) -> u32 {
    let files_size: u32 = node
        .data()
        .files
        .clone()
        .into_iter()
        .map(|f| f.1)
        .sum::<u32>();
    let total_size = node.children().map(|kid| dir_size(kid, dirs)).sum::<u32>() + files_size;

    dirs.push(total_size);
    return total_size;
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut tree = TreeBuilder::new()
        .with_root(Directory {
            name: "/".to_string(),
            files: HashSet::new(),
        })
        .build();
    let mut lines = input.lines();

    let mut state = State {
        current_dir: tree.root_id().unwrap(),
    };

    loop {
        let cmd = lines.next();
        match cmd {
            Some(s) => {
                if s.starts_with("$ cd") {
                    let arg = s.strip_prefix("$ cd ");
                    match arg {
                        Some("..") => {
                            state.current_dir = tree
                                .get(state.current_dir)
                                .unwrap()
                                .parent()
                                .unwrap()
                                .node_id()
                        }
                        Some("/") => state.current_dir = tree.root_id().unwrap(),
                        Some(dir) => {
                            state.current_dir = tree
                                .get(state.current_dir)
                                .unwrap()
                                .children()
                                .find(|c| c.data().name == dir)
                                .unwrap()
                                .node_id()
                        }
                        _ => panic!(),
                    }
                } else if s.starts_with("$ ls") {
                    continue;
                } else if s.starts_with("dir") {
                    tree.get_mut(state.current_dir).unwrap().append(Directory {
                        name: s.strip_prefix("dir ").unwrap().to_string(),
                        files: HashSet::new(),
                    });
                } else {
                    let (size, filename) = s.split_once(" ").unwrap();
                    tree.get_mut(state.current_dir)
                        .unwrap()
                        .data()
                        .files
                        .insert((filename.to_string(), (size.parse::<u32>().unwrap())));
                }
            }
            None => break,
        }
    }

    // let mut s = String::new();
    // tree.write_formatted(&mut s).unwrap();
    // println!("{s}");

    let mut dirs: Vec<u32> = Vec::new();
    dir_size(tree.root().unwrap(), &mut dirs);
    dirs.iter().filter(|n| **n < 100000).sum()
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> u32 {
    let mut tree = TreeBuilder::new()
        .with_root(Directory {
            name: "/".to_string(),
            files: HashSet::new(),
        })
        .build();
    let mut lines = input.lines();

    let mut state = State {
        current_dir: tree.root_id().unwrap(),
    };

    loop {
        let cmd = lines.next();
        match cmd {
            Some(s) => {
                if s.starts_with("$ cd") {
                    let arg = s.strip_prefix("$ cd ");
                    match arg {
                        Some("..") => {
                            state.current_dir = tree
                                .get(state.current_dir)
                                .unwrap()
                                .parent()
                                .unwrap()
                                .node_id()
                        }
                        Some("/") => state.current_dir = tree.root_id().unwrap(),
                        Some(dir) => {
                            state.current_dir = tree
                                .get(state.current_dir)
                                .unwrap()
                                .children()
                                .find(|c| c.data().name == dir)
                                .unwrap()
                                .node_id()
                        }
                        _ => panic!(),
                    }
                } else if s.starts_with("$ ls") {
                    continue;
                } else if s.starts_with("dir") {
                    tree.get_mut(state.current_dir).unwrap().append(Directory {
                        name: s.strip_prefix("dir ").unwrap().to_string(),
                        files: HashSet::new(),
                    });
                } else {
                    let (size, filename) = s.split_once(" ").unwrap();
                    tree.get_mut(state.current_dir)
                        .unwrap()
                        .data()
                        .files
                        .insert((filename.to_string(), (size.parse::<u32>().unwrap())));
                }
            }
            None => break,
        }
    }

    // let mut s = String::new();
    // tree.write_formatted(&mut s).unwrap();
    // println!("{s}");

    let mut dirs: Vec<u32> = Vec::new();
    let root_size = dir_size(tree.root().unwrap(), &mut dirs);
    let current_free_space = FS_SIZE - root_size;
    *dirs
        .iter()
        .sorted()
        .find(|n| current_free_space + **n >= NEEDED_SPACE)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test_case(EXAMPLE_INPUT => 95437)]
    fn part1_example(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(EXAMPLE_INPUT => 24933642)]
    fn part2_example(input: &str) -> u32 {
        solve_part2(input)
    }

    #[test_case(&get_input("day7") => 1307902)]
    fn part1_real(input: &str) -> u32 {
        solve_part1(input)
    }

    #[test_case(&get_input("day7") => 7068748)]
    fn part2_real(input: &str) -> u32 {
        solve_part2(input)
    }
}
