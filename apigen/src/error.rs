use api_analyzer::AnalyzerError;
use api_parser::ParserError;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiGenCore(ParserError),
    Analyzer(AnalyzerError),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Error {
        Error::ApiGenCore(err)
    }
}

impl From<AnalyzerError> for Error {
    fn from(err: AnalyzerError) -> Error {
        Error::Analyzer(err)
    }
}
