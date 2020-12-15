use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct NoSuchElementError;

impl fmt::Display for NoSuchElementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No element was found.")
    }
}

impl error::Error for NoSuchElementError {}