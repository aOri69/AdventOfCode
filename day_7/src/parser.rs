use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, not_line_ending},
    combinator::map,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

/// Struct represents list terminal command
/// `ls`
#[derive(Debug, PartialEq, Eq)]
struct Ls;

/// Struct represents change directory command
/// `cd path`. Path is stored as String
#[derive(Debug, PartialEq, Eq)]
struct Cd(String);

/// Enum with all the available commands in this task
/// `ls` and `cd`
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ls,
    Cd(String),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(value: Cd) -> Self {
        Command::Cd(value.0)
    }
}

/// Type of terminal output after `ls` command
/// Can be either `directory` or `file`
#[derive(Debug, PartialEq, Eq)]
pub enum Entry {
    Dir(String),
    File(u64, String),
}

/// Each line of terminal input/output
/// can be either a command `$ command`
/// or a command `ls` output
#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_ls(input: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(input)
}

fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(
        preceded(terminated(tag("cd"), multispace1), not_line_ending),
        |s: &str| Cd(s.to_string()),
    )(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let shell_line_begin = terminated(tag("$"), multispace1);
    let cmd_ls = map(parse_ls, Into::into);
    let cmd_cd = map(parse_cd, Into::into);

    preceded(shell_line_begin, alt((cmd_ls, cmd_cd)))(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let dir = map(
        preceded(terminated(tag("dir"), multispace1), not_line_ending),
        |s: &str| Entry::Dir(s.to_string()),
    );
    let file = map(
        separated_pair(
            nom::character::complete::u64,
            tag(" "),
            map(not_line_ending, |p: &str| p.to_string()),
        ),
        |(s, p)| Entry::File(s, p),
    );

    alt((dir, file))(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(input)
}

#[cfg(test)]
mod tests {
    // use nom::{combinator::all_consuming, Finish};

    use nom::{combinator::all_consuming, Finish};

    use super::*;

    #[test]
    fn test_ls() {
        assert_eq!(parse_ls("lsasdasdas"), Ok(("asdasdas", Ls)));
    }

    #[test]
    fn test_cd() {
        assert_eq!(
            parse_cd("cd /home\r\n"),
            Ok(("\r\n", Cd("/home".to_string())))
        );
    }

    #[test]
    fn test_command() {
        assert_eq!(
            parse_command("$ cd /"),
            Ok(("", Command::Cd("/".to_string())))
        );
    }

    #[test]
    fn test_full_input() {
        let input = "$ cd /
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

        let res = input
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect::<Vec<_>>();

        res.iter().for_each(|l| println!("{l:?}"));
    }
}
