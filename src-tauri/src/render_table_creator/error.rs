#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Other(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Self::Other(e)
    }
}