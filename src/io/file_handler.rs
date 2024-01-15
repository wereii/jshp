use log::trace;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileHandler {
    root_dir: PathBuf,
}

impl FileHandler {
    pub fn new(path: &str) -> Result<Self, FileHandlerError> {
        let root_dir = PathBuf::from(path).canonicalize().unwrap();

        if !root_dir.exists() || !root_dir.is_dir() {
            return Err(FileHandlerError::InvalidServeDir(path.to_string()));
        }
        trace!("Serving from {:?}", root_dir);
        Ok(Self { root_dir })
    }

    pub fn get_file(&self, path: &str) -> Result<PathBuf, FileHandlerError> {
        let file = self.root_dir.join(
            Path::new(path)
                .strip_prefix("/")
                .expect("all requests should start with /"),
        );

        trace!("Trying to get file {:?}", file);

        if !file.exists() || !file.is_file() {
            return Err(FileHandlerError::FileDoesntExist(path.to_string()));
        }
        Ok(file)
    }

    pub fn read_to_string(&self, path: &str) -> Result<String, FileHandlerError> {
        let file = self.get_file(path)?;
        Ok(std::fs::read_to_string(file).expect("Existence should be already checked"))
    }

    pub fn read_bytes(&self, path: &str) -> Result<Vec<u8>, FileHandlerError> {
        let file = self.get_file(path)?;
        Ok(std::fs::read(file).expect("Existence should be already checked"))
    }
}

#[derive(Debug, Clone)]
pub enum FileHandlerError {
    InvalidServeDir(String),
    FileDoesntExist(String),
}

impl std::error::Error for FileHandlerError {}

impl std::fmt::Display for FileHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileHandlerError::InvalidServeDir(e) => write!(f, "invalid serve dir: {}", e),
            FileHandlerError::FileDoesntExist(e) => write!(f, "file doesn't exist: {}", e),
        }
    }
}
