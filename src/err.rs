use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    desc: String,
}

impl Error {
    #[inline]
    pub fn new(scope: &str, msg: &str) -> Self {
        Self {
            desc: format!("{scope}: {msg}"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl std::error::Error for Error {}
