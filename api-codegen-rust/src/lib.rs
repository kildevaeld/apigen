#[macro_use]
extern crate serde_derive;
extern crate api_analyzer;
extern crate api_codegen;
#[macro_use]
extern crate api_extensions;
extern crate api_parser;
extern crate bytes;
extern crate handlebars;
extern crate heck;
extern crate rayon;
extern crate serde;
// #[macro_use]
// extern crate log;

mod code_generator;
mod rust_pass;
mod template;
mod visitor;

pub use code_generator::RustCodeGenerator;
pub use rust_pass::{rust, RustPass};

#[derive(Default, Debug)]
pub struct RustPlugin;

impl api_extensions::Extension for RustPlugin {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn passes(&self) -> Option<Vec<Box<dyn api_analyzer::Pass>>> {
        let mut passes = api_analyzer::default_passes();
        passes.push(Box::new(RustPass::new()));
        Some(passes)
    }

    fn generator(&self) -> Option<Box<dyn api_codegen::CodeGenerator>> {
        Some(Box::new(RustCodeGenerator::new()))
    }
}

declare_extension!(RustPlugin, RustPlugin::default);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
