/// Struct represents list terminal command
/// `ls`
#[derive(Debug, PartialEq, Eq)]
pub struct Ls;

/// Struct represents change directory command
/// `cd path`. Path is stored as String
#[derive(Debug, PartialEq, Eq)]
pub struct Cd(pub String);

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
