#[derive(Debug)]
pub struct CommandError(nom::error::Error<String>);
impl From<nom::error::Error<String>> for CommandError {
    fn from(value: nom::error::Error<String>) -> Self {
        Self(value)
    }
}
impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
impl std::error::Error for CommandError {}
