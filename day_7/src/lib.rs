#[allow(dead_code, unused_variables, unused_assignments)]
pub mod directory;
mod parser;
mod structs;

use crate::directory::{Node, PrettyNode};
use parser::get_parsed_lines;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Actual soution for Day 7 Part 1 of Advent of Code 2022
pub fn part_1(input: &str) -> u64 {
    let parsed_lines = get_parsed_lines(input);

    let root = Rc::new(RefCell::new(Node::new("/", Weak::new())));

    let mut current_dir = Rc::clone(&root);

    let mut directories = vec![Rc::clone(&current_dir)];

    for line in parsed_lines {
        // println!("{line:?}");
        match line {
            structs::Line::Command(cmd) => match cmd {
                structs::Command::Ls => continue,
                structs::Command::Cd(path) => match path.as_str() {
                    "/" => current_dir = Rc::clone(&root),
                    ".." => {
                        let parent = current_dir.borrow().parent().upgrade().unwrap();
                        current_dir = parent;
                    }
                    path => {
                        let child_dir = current_dir
                            .borrow_mut()
                            .children_mut()
                            .iter()
                            .find(|x| x.borrow().name() == path)
                            .unwrap()
                            .clone();
                        current_dir = child_dir;
                    }
                },
            },
            structs::Line::Entry(entry) => match entry {
                structs::Entry::Dir(path) => {
                    let new_child_dir =
                        Rc::new(RefCell::new(Node::new(&path, Rc::downgrade(&current_dir))));
                    directories.push(Rc::clone(&new_child_dir));
                    current_dir.borrow_mut().children_mut().push(new_child_dir);
                }
                structs::Entry::File(size, name) => {
                    let new_child_file =
                        Rc::new(RefCell::new(Node::new(&name, Rc::downgrade(&current_dir))));
                    new_child_file.borrow_mut().set_size(size);
                    current_dir.borrow_mut().children_mut().push(new_child_file);
                }
            },
        };
    }

    drop(current_dir);

    println!("{:#?}", PrettyNode(&root));
    println!("root size: {}", &root.borrow().size());

    directories
        .iter()
        .map(|d| d.borrow().size())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum()
}

pub fn part_2(_input: &str) {
    todo!("part2");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 95437);
    }

    #[test]
    fn test_part_2() {
        part_2(INPUT);
    }
}
