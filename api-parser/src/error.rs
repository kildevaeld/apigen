use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Syntax(String),
}

impl error::Error for ParserError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "Parser error"),
        }
    }
}
