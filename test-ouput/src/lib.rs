#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
extern crate api_support;

pub use api_support::error;

mod arangodb;
pub use arangodb::*;

mod models;
pub use models::*;

mod test;
pub use test::*;
