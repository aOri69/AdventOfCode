use nom::{
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

pub fn parse_command(_input: &str) -> IResult<&str, Command> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ls() {
        let res = parse_ls("lsasdasdas");
        assert_eq!(res, Ok(("asdasdas", Ls)));
    }

    #[test]
    fn test_cd() {
        let res = parse_cd("cd /home\r\n");
        assert_eq!(res, Ok(("\r\n", Cd("/home".to_string()))));
    }
}
