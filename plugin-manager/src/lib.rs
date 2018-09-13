extern crate libloading;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate uuid;

extern crate serde;

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

pub mod error;
mod plugin_manager;

pub use plugin_manager::{Plugin, PluginManager, PluginManager2};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
