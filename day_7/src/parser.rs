use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, not_line_ending},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};

use crate::structs::{Cd, Command, Entry, Line, Ls};

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

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(input)
}

/// High level function to turn `&str` input
/// into the Iterator of `parser::Line` items
/// which could be either `Command` or `Entry`
pub fn get_parsed_lines(input: &str) -> impl Iterator<Item = Line> + '_ {
    input
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
}

#[cfg(test)]
mod tests {
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
    fn test_parse_ls() {
        let result = parse_ls("ls");
        assert_eq!(result, Ok(("", Ls)));
    }

    #[test]
    fn test_parse_cd() {
        let result = parse_cd("cd path");
        assert_eq!(result, Ok(("", Cd("path".to_string()))));
    }

    #[test]
    fn test_parse_command_ls() {
        let result = parse_command("$ ls");
        assert_eq!(result, Ok(("", Command::Ls)));
    }

    #[test]
    fn test_parse_command_cd() {
        let result = parse_command("$ cd path");
        assert_eq!(result, Ok(("", Command::Cd("path".to_string()))));
    }

    #[test]
    fn test_parse_entry_dir() {
        let result = parse_entry("dir directory_name");
        assert_eq!(result, Ok(("", Entry::Dir("directory_name".to_string()))));
    }

    #[test]
    fn test_parse_entry_file() {
        let result = parse_entry("1234 file_name.txt");
        assert_eq!(
            result,
            Ok(("", Entry::File(1234, "file_name.txt".to_string())))
        );
    }

    #[test]
    fn test_parse_line_command() {
        let result = parse_line("$ ls");
        assert_eq!(result, Ok(("", Line::Command(Command::Ls))));
    }

    #[test]
    fn test_parse_line_entry() {
        let result = parse_line("dir directory_name");
        assert_eq!(
            result,
            Ok(("", Line::Entry(Entry::Dir("directory_name".to_string()))))
        );
    }

    #[test]
    fn test_get_parsed_lines() {
        let input = "$ ls\ndir directory_name";
        let expected = vec![
            Line::Command(Command::Ls),
            Line::Entry(Entry::Dir("directory_name".to_string())),
        ];
        let parsed_lines: Vec<Line> = get_parsed_lines(input).collect();
        assert_eq!(parsed_lines, expected);
    }
}
