#[macro_use]
extern crate serde_derive;
extern crate api_analyzer;
extern crate api_codegen;
extern crate api_parser;
extern crate bytes;
extern crate handlebars;
extern crate heck;
extern crate rayon;
extern crate serde;
mod code_generator;
mod rust_pass;
mod template;
mod visitor;

pub use code_generator::RustCodeGenerator;
pub use rust_pass::{rust, RustPass};

// #[derive(Default, Debug)]
// pub struct RustPlugin {}

// impl Plugin for RustPlugin {
//     fn name(&self) -> &'static str {
//         "Rust"
//     }

//     fn on_plugin_load(&self) {
//         println!("plugin loaded");
//     }
// }

// impl api_codegen::CodeGeneratorPlugin for RustPlugin {
//     fn register(
//         &self,
//         builder: &mut api_codegen::CodeGeneratorPluginBuilder,
//     ) -> api_codegen::Result<()> {
//         Ok(())
//     }
// }

// declare_plugin!(RustPlugin, RustPlugin::default);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
