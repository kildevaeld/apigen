extern crate api_analyzer;
extern crate api_parser;
extern crate bytes;
extern crate plugin_manager;
extern crate uuid;
#[macro_use]
extern crate error_chain;
extern crate rayon;

mod code_generator;
pub mod error;
mod repository;
//pub mod unit;
pub use code_generator::*;
pub use error::*;
pub use repository::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
