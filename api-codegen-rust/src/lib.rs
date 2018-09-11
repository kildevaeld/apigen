#[macro_use]
extern crate serde_derive;
extern crate api_analyzer;
extern crate api_codegen;
extern crate api_parser;
extern crate bytes;
extern crate handlebars;

extern crate heck;
extern crate serde;

mod code_generator;
mod rust_pass;
mod template;
mod visitors;

pub use code_generator::RustCodeGenerator;
pub use rust_pass::{rust, RustPass};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
