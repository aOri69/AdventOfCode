use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1, not_line_ending},
    combinator::map,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Ls;

#[derive(Debug, PartialEq, Eq)]
pub struct Cd(String);

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Ls(Ls),
    Cd(Cd),
}

pub fn parse_ls(input: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(input)
}

pub fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(
        preceded(terminated(tag("cd"), multispace1), not_line_ending),
        |s: &str| Cd(s.to_string()),
    )(input)
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    let shell_line_begin = terminated(tag("$"), multispace1);
    let cmd_ls = map(parse_ls, Command::Ls);
    let cmd_cd = map(parse_cd, Command::Cd);

    preceded(shell_line_begin, alt((cmd_ls, cmd_cd)))(input)
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
            Ok(("", Command::Cd(Cd("/".to_string()))))
        );
    }
}
