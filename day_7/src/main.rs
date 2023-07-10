use std::{cell::RefCell, rc::Rc};

use no_space_left_on_device::{directory::Node, get_parsed_lines, Command, Entry, Line};

fn main() {
    let input = include_str!("../input_test.txt");
    let parsed_lines = get_parsed_lines(input);

    // "/" root directory
    let root = Rc::new(RefCell::new(Node::default()));
    // current iteration directory
    // initially - root one
    let mut node = Rc::clone(&root);

    for line in parsed_lines {
        println!("{line:?}");

        match line {
            Line::Command(command) => match command {
                Command::Ls => continue,
                Command::Cd(dir) => match dir.as_str() {
                    "/" => todo!(),
                    ".." => todo!(),
                    _ => todo!(),
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_dir) => todo!(),
                Entry::File(_size, _name) => todo!(),
            },
        }
    }
}
