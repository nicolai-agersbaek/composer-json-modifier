use std::fs;
use std::io;
use std::path::Path;

pub(crate) trait PathAsserts {
    fn assert_exists(&self) -> Result<&Path, io::Error>;
    fn assert_is_file(&self) -> Result<&Path, io::Error>;
}

impl PathAsserts for Path {
    fn assert_exists(&self) -> Result<&Path, io::Error> {
        if !self.exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", self.display()),
                )
            );
        }

        Ok(self)
    }

    fn assert_is_file(&self) -> Result<&Path, io::Error> {
        if !self.is_file() {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Path is not a file: {}", self.display()),
                )
            );
        }

        Ok(self)
    }
}

pub(crate) fn get_file_path(s: &str) -> Result<&Path, io::Error> {
    Path::new(s).assert_exists()?.assert_is_file()
}

pub(crate) fn get_file_contents(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(get_file_path(file_name)?)
}
