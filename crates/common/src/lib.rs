pub mod utils;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct CraftyError {
    message: String,
}

impl CraftyError {
    pub fn new<T: Into<String>>(message: T) -> CraftyError {
        CraftyError {
            message: message.into(),
        }
    }
}

impl Display for CraftyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = core::result::Result<T, CraftyError>;
