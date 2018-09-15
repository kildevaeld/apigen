use futures::{Future, Stream};
use reqwest::async::{Client, Decoder};
use reqwest::header::CONTENT_TYPE;
use reqwest::{Method, Request};
use std::mem;

use api_support::error;
use api_support::utils;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Response<T: Serialize + Deserialize> {
  error: bool,
  code: i16,
  result: T
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
  username: String,
  passwd: Option<String>,
  active: bool,
  extra: HashMap<String, Any>,
  array: Vec<String>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Other {
  test: OtherTest
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OtherTest {
  rapper: Option<String>,
  extra: Vec<String>
}

