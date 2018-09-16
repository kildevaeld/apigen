extern crate api_analyzer;
extern crate api_codegen;
extern crate api_extensions;
extern crate api_parser;
#[macro_use]
extern crate error_chain;

mod core;
mod error;
mod generator;
mod repository;

pub use core::*;
pub use error::*;
pub use generator::*;
pub use repository::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
