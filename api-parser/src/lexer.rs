use error::{ParserError, Result};
#[allow(unused_imports)]
use pest::{iterators, Error as PestError, Parser};
use std::error::Error;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct Lexer;

pub type Pairs<'a> = iterators::Pairs<'a, Rule>;
pub type Pair<'a> = iterators::Pair<'a, Rule>;

impl<'a> From<PestError<'a, Rule>> for ParserError {
    fn from(err: PestError<'a, Rule>) -> ParserError {
        println!("{}", err);
        ParserError::Syntax(String::from(err.description()))
    }
}

pub fn tokenize(input: &str) -> Result<Pairs> {
    Ok(Lexer::parse(Rule::main, &input)?)
}
