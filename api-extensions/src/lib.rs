#[macro_use]
pub extern crate plugins;
#[macro_use]
extern crate error_chain;
extern crate api_analyzer;
extern crate api_codegen;
use plugins::ResultExt;

pub mod error {
    error_chain!{}
}

pub trait ExtensionBuilder {
    fn register_pass(
        &mut self,
        name: &str,
        pass: Fn() -> Vec<Box<dyn api_analyzer::Pass>>,
    ) -> &mut ExtensionBuilder;
    fn register_codegen(
        &mut self,
        name: &str,
        pass: Box<dyn api_codegen::CodeGenerator>,
    ) -> &mut ExtensionBuilder;
}

use error::Result;
plugin_manager!{
    manager_name = Extensions;
    pub trait Extension {
        fn name(&self) -> Result<&'static str>;
        fn register(&self, builder: &mut ExtensionBuilder) -> Result<()>;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
