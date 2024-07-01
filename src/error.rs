use std::fmt::Display;

#[derive(Debug)]
pub enum ErrorKind {
    Gui,
    UrlProcessing,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Gui => "GUI",
            Self::UrlProcessing => "Invalid URL",
        })
    }
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Self {
        Self {
            kind,
            msg: msg.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: ({}) {}", self.kind.to_string(), self.msg)
    }
}
