#[macro_use]
extern crate serde_derive;

extern crate serde;
//extern crate serde_json;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod error;
mod lexer;
mod parser;

pub mod expressions;
pub use error::{ParserError, Result};
pub use lexer::{tokenize, Pair};
pub use parser::build_ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
