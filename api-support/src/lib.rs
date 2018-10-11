pub extern crate futures;
pub extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
pub extern crate tokio;
pub extern crate url;
#[macro_use]
extern crate error_chain;

pub mod auth;
pub mod error;
pub mod macros;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
