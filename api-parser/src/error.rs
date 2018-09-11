use std::result;

pub type Result<T> = result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Syntax(String),
}
