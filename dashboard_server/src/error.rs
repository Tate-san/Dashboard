use std::io;

#[derive(Debug)]
pub enum ServerError{
    IoError(io::Error),
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
        ServerError::IoError(error)
    }
}
