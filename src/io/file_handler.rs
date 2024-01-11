use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileHandler {
    root_dir: PathBuf,
}

impl FileHandler {
    pub fn new(path: &str) -> Result<Self, FileHandlerError> {
        let root_dir = PathBuf::from(path);

        if !root_dir.exists() || !root_dir.is_dir() {
            return Err(FileHandlerError::InvalidServeDir(path.to_string()));
        }

        Ok(Self { root_dir })
    }

    pub fn get_file(&self, path: &str) -> Result<PathBuf, FileHandlerError> {
        let mut file = self.root_dir.clone();
        file.push(path);

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

impl std::fmt::Display for FileHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileHandlerError::InvalidServeDir(e) => write!(f, "Invalid serve dir: {}", e),
            FileHandlerError::FileDoesntExist(e) => write!(f, "File doesn't exist: {}", e),
        }
    }
}
