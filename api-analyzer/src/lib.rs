extern crate api_parser;
#[macro_use]
extern crate log;

mod analyze;
mod common;
mod error;
pub mod passes;
mod scope;
mod types;

pub use analyze::{analyze, analyze_file, default_passes};
pub use error::{AnalyzerError, Result};
pub use scope::{Scope, ScopeType};
pub use types::{Pass, PassList};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
