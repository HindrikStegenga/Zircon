use std::{
    error::Error,
    fmt::{write, Display},
};

#[derive(Debug)]
pub enum VfsError {
    MountpointNotFound,
    FileNotFound,
    Other(Box<dyn Error>),
    Io(std::io::Error),
}

impl Display for VfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VfsError::Other(e) => write!(f, "{}", e),
            VfsError::FileNotFound => write!(f, "File not found."),
            VfsError::MountpointNotFound => write!(f, "Invalid mount point."),
            VfsError::Io(err) => err.fmt(f),
        }
    }
}
impl Error for VfsError {}

impl From<std::io::Error> for VfsError {
    fn from(e: std::io::Error) -> Self {
        VfsError::Io(e)
    }
}
