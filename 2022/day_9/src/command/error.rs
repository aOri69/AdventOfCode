#[derive(Debug)]
pub struct CommandError(String);

impl<T> From<nom::error::Error<T>> for CommandError
where
    T: std::fmt::Display,
{
    fn from(value: nom::error::Error<T>) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CommandError {}
