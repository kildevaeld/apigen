use api_parser::ParserError;
use std::error;
use std::fmt;
use std::io;
use std::result;
pub type Result<T> = result::Result<T, AnalyzerError>;

#[derive(Debug)]
pub enum AnalyzerError {
    Import(String),
    Parser(ParserError),
    Io(io::Error),
    Reference(String),
    TypeError(String),
}

impl From<io::Error> for AnalyzerError {
    fn from(err: io::Error) -> AnalyzerError {
        AnalyzerError::Io(err)
    }
}

impl From<ParserError> for AnalyzerError {
    fn from(err: ParserError) -> AnalyzerError {
        AnalyzerError::Parser(err)
    }
}

impl error::Error for AnalyzerError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "AnalyzeError"),
        }
    }
}
