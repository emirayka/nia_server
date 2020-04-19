#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn unknown() -> Error {
        Error {
            kind: ErrorKind::Unknown,
        }
    }
}

