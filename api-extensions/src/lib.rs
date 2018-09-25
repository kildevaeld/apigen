#[macro_use]
pub extern crate plugins;
#[macro_use]
extern crate error_chain;
extern crate api_analyzer;
extern crate api_codegen;

mod error;

pub use error::*;
pub use plugins::PluginManager;

use std::fmt::Debug;

pub trait Extension: Debug {
    fn name(&self) -> &'static str;
    fn passes(&self) -> Option<Vec<Box<dyn api_analyzer::Pass>>>;
    fn generator(&self) -> Option<Box<dyn api_codegen::CodeGenerator>>;
}

build_plugin_manager!(Extension, ExtensionManager);

#[macro_export]
macro_rules! declare_extension {
    ($plugin_type:ty, $constructor:path) => {
        use api_extensions::Extension;
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut Extension {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$plugin_type> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
