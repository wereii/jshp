use crate::io::FileHandlerError;

#[derive(Debug, Clone)]
pub enum Error {
    FileHandlerError(FileHandlerError),
}

impl From<FileHandlerError> for Error {
    fn from(error: FileHandlerError) -> Self {
        Self::FileHandlerError(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FileHandlerError(e) => write!(f, "File error: {}", e),
        }
    }
}
