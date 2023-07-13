pub mod directory;
mod parser;
mod structs;

use std::collections::BTreeMap;

use parser::get_parsed_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Directory(String);
#[derive(Debug)]
struct File(usize, String);
#[derive(Debug)]
enum FileSystemEntry {
    Dir(Directory),
    File(File),
}

pub fn part_1(input: &str) -> u32 {
    let parsed_lines = get_parsed_lines(input);

    let mut directories: BTreeMap<Directory, Vec<FileSystemEntry>> = BTreeMap::new();
    // adding root
    directories.insert(Directory("/".to_owned()), vec![]);

    let mut current_dir = directories.entry(Directory("/".to_owned()));

    for line in parsed_lines {
        println!("{line:?}");
        match line {
            structs::Line::Command(cmd) => match cmd {
                structs::Command::Ls => continue,
                structs::Command::Cd(path) => {
                    directories.entry(Directory(path.clone())).or_insert(vec![]);
                    current_dir = directories.entry(Directory(path));
                }
            },
            structs::Line::Entry(entry) => continue,
            // structs::Line::Entry(entry) => match entry {
            //     structs::Entry::Dir(path) => directories.entry(path),
            //     structs::Entry::File(size, name) => todo!(),
            // },
        };
    }
    dbg!(directories);
    todo!("part1");
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
