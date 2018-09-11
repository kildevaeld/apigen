use api_parser::ParserError;
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
