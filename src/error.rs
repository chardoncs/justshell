#[derive(Debug)]
pub enum ErrorKind {
    Gui,
    UrlAbsent,
    UrlProcessing,
}

impl ToString for ErrorKind {
    fn to_string(&self) -> String {
        match self {
            Self::Gui => "GUI",
            Self::UrlAbsent => "No URLs",
            Self::UrlProcessing => "Invalid URL",
        }.to_string()
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

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("error: ({}) {}", self.kind.to_string(), self.msg)
    }
}
