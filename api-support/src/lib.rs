extern crate futures;
extern crate serde_derive;
//extern crate http;
//extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
#[macro_use]
extern crate error_chain;
pub mod error;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
